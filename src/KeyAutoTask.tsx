import {Text, Box, ScrollArea, Section,} from '@radix-ui/themes';
import React, {useEffect} from "react";
import {OperationPanel} from "./components/OperationPanel";
import {invoke} from "@tauri-apps/api/core";

import SeparatorComponent from "@/components/Separator";
import {
    EventInfo,
    useRecordEventStore,
    useRecordProcessId,
    useRecordStore,
    useRunTaskCount,
    useStopExecution
} from "@/store";
import {globalListenKeyDown, globalStopListenKeyDown, readRecordFile} from "@/utils/GlobalListen.ts";
import {Toaster} from "react-hot-toast";
import {emptyRecordFileToast} from "@/components/Toast.tsx";
import {convertRawToRecordType, RecordEventsArr, RecordEventsType, RecordType} from "@/utils/EventsType.ts";

export function KeyMouseEventViewBox(_props: {
    children: React.ReactNode;
}) {
    const recordEvents = useRecordEventStore(state => state.events);

    const isRecording = useRecordStore(state => state.isRecording);
    const resetEvents = useRecordEventStore(state => state.ResetEvents);

    const setEvents = useRecordEventStore(state => state.SetEvents);
    const initRecordEvents = () => {
        resetEvents();
        invoke("read_record_key_from_file").then((result) => {
            if (result && typeof result === "string") {
                const eventsObj = JSON.parse(result) as RecordEventsType;
                const events = eventsObj.events.map((e: RecordEventsArr) => {
                    const currentType = e.event_type;
                    const current = convertRawToRecordType(currentType) as RecordType;
                    const now = new Date();
                    const timestamp = now.toLocaleString();
                    switch (current.type) {
                        case    "KeyPress": {
                            return {type: "keydown", detail: current.value, timestamp}
                        }
                        case    "KeyRelease": {
                            return {type: "keyup", detail: current.value, timestamp}
                        }
                        case    "ButtonLeft": {
                            return {
                                type: "mousedown",
                                detail: `ButtonLeft at (${current.value[0]}, ${current.value[1]})`,
                                timestamp: timestamp
                            }
                        }
                        case    "ButtonRight": {
                            return {
                                type: "contextmenu",
                                detail: `ButtonRight at (${current.value[0]}, ${current.value[1]})`
                                ,
                                timestamp: timestamp

                            }
                        }
                        case    "MouseWheel": {
                            return {
                                type: "wheel",
                                detail: `MouseWheel at (${current.value[0]}, ${current.value[1]})`,
                                timestamp: timestamp

                            }
                        }
                        default: {
                            return {
                                type: "unknown",
                                detail: "unknown",
                                timestamp: timestamp
                            }
                        }
                    }
                }) as EventInfo[];

                console.warn("current events:", events.length)
                setEvents(events);

            } else {
                emptyRecordFileToast();
            }
        })

    }

    const handleRightClick = (e: MouseEvent) => {
        e.preventDefault();
    };

    const setProcessId = useRecordProcessId(state => state.setProcessId);
    const pid = useRecordProcessId(state => state.processId);
    //Todo  添加停止任务执行
    useEffect(() => {
        console.log("current isRecording: " + isRecording)
        console.log("current pid : " + pid)
        if (isRecording) {
            window.addEventListener("contextmenu", handleRightClick);
            globalListenKeyDown(setProcessId);
            return () => {
                window.removeEventListener("contextmenu", handleRightClick);
                globalStopListenKeyDown(pid, setProcessId);
            };
        } else {
            initRecordEvents();
            return () => {
            };
        }
    }, [isRecording, pid])

    useEffect(() => {
        initRecordEvents();
    }, []);

    return (
        <Box
            width="full"
            maxHeight={"400px"}
            style={{
                overflowY: "auto",
                border: "1px solid var(--gray-a4)",
                borderRadius: "var(--radius-3)",
                padding: "1rem",
                backgroundColor: "var(--gray-a2)",
            }}
        >
            <Box style={{position: "sticky", top: 0, zIndex: 1}}>
                <Text weight="bold">事件记录</Text>
            </Box>
            <Box style={{padding: "1rem", paddingTop: 0}}>
                <ScrollArea type="always" scrollbars="vertical" radius="full">
                    {recordEvents.slice().reverse().map((e, i) => (
                        <Box key={i} p="2" pr="8">
                            <Text as="p" color="gray">
                                [{e.timestamp}] {e.type.toUpperCase()} - {e.detail}
                            </Text>
                        </Box>
                    ))}

                </ScrollArea>
            </Box>
        </Box>
    );
}


export function KeyAutoTask() {
    const isRecording = useRecordStore(state => state.isRecording);
    const setIsRecording = useRecordStore(state => state.setIsRecording);
    const clearEvents = useRecordEventStore(state => state.clearEvents);
    const startRecording = () => {
        if (!isRecording) {
            invoke("init_record_key_file").then(_r => {
                console.dir("clear record file")
            });
        }
        if (!isRecording) {
            setIsRecording(true);
        }
    }

    const ClearEvent = () => {
        clearEvents();
    }

    const stopRecording = () => {
        if (isRecording) {
            setIsRecording(false);
        }
    }

    const taskCounts = useRunTaskCount(state => state.runTaskCount);
    const needStop = useStopExecution(state => state.needStop);
    const executeRecording = () => {
        if (isRecording) {
            alert("请先停止录制")
            return;
        }
        readRecordFile(emptyRecordFileToast, taskCounts, needStop);
    }

    return (
        <>
            <OperationPanel onStartRecording={startRecording}
                            onStopRecording={stopRecording}
                            onExecuteRecording={executeRecording}
                            ClearEvent={ClearEvent}
            />
            <SeparatorComponent/>
            <Box
                py="8"
                style={{backgroundColor: "var(--gray-a2)", borderRadius: "var(--radius-3)"}}
            >
                <KeyMouseEventViewBox
                >
                    <Section size="2"/>
                </KeyMouseEventViewBox>
                <Toaster
                    position="top-center"
                    reverseOrder={false}
                />
            </Box>

        </>

    );
}
