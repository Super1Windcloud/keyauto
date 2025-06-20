import {Box, Button, Container, Flex} from "@radix-ui/themes";
import {Label} from "radix-ui";
import {StartExecuteRecordTask} from "@/components/StartExecuteRecord"
import {useEffect, useRef, useState} from "react";
import {useExecutionStatus, useRecordStore, useRunTaskCount} from "@/store";
import {clearToast, executeToast, notifyStartRecord, notifyStopRecord, stopExecuteToast} from "@/components/Toast.tsx";
import {Toaster} from "react-hot-toast";

interface OperationPanelProps {
    onStartRecording?: () => void;
    onStopRecording?: () => void;
    onExecuteRecording?: () => void;
    ClearEvent?: () => void;
}


export function OperationPanel(props: OperationPanelProps) {
    const width = (window.innerWidth).toString();
    const height = (window.innerHeight / 3 + 50).toString();
    const isRecord = useRecordStore(state => state.isRecording);
    const [stopToast, setStopToast] = useState(false);
    const [startToast, setStartToast] = useState(false);
    const ShowStopToast = () => {
        if (!stopToast) {
            setStopToast(true);
        }
    }
    useEffect(() => {
        if (isRecord) {
            setStartToast(true);
        }
    }, [isRecord])
    const clickEvent = props.onExecuteRecording || (() => {
    });
    const executeStatus = useExecutionStatus(status => status.isExecuting);
    // @ts-ignore
    const _origin = executeStatus;
    const setExecute = useExecutionStatus(status => status.setIsExecuting)
    const setExecuteStatus = () => {
        if (executeStatus) {
            setExecute(false)
        }
    }

    useEffect(() => {
        if (startToast) {
            notifyStartRecord();
            setStopToast(false);
        }
    }, [startToast])
    useEffect(() => {
        if (stopToast) {
            notifyStopRecord();
            setStartToast(false);
        }
    }, [stopToast]);

    const isFirstRun = useRef(0);
    const   setRunTasks= useRunTaskCount(count => count.setRunTaskCount);

    useEffect(() => {
        if (isFirstRun.current <= 2) {
            isFirstRun.current += 1;
            return; // 跳过首次执行
        }
        if (executeStatus) {
            executeToast();
        } else if (!executeStatus) {
            stopExecuteToast();
        }
    }, [executeStatus])


    ///todo 添加执行延迟设置
    return (
        <Box width={width + "px"} height={height + "px"} py="8"
             style={{
                 background: "var(--gray-a2)", borderRadius: "var(--radius-3)"
                 , marginRight: "10px", padding: "10px", boxShadow: "var(--shadow-1)"
                 , left: "10px"
             }
             }>
            <Container align="center">
                <h1 className="text-white text-3xl font-bold mb-5">操作面板</h1>
                <Flex gap="2" align="center" justify="center">
                    <div className="flex flex-wrap items-center gap-[15px] px-5 mb-5">
                        <Label.Root
                            className="text-[15px] font-medium leading-[35px] text-white"
                            htmlFor="firstName"
                        >
                            运行次数
                        </Label.Root>
                        <input
                            className="inline-flex h-[35px] w-[200px] appearance-none items-center justify-center rounded bg-blackA2 px-2.5 text-[15px] leading-none text-white shadow-[0_0_0_1px] shadow-blackA6 outline-none selection:bg-blackA6 selection:text-white focus:shadow-[0_0_0_2px_black]"
                            type="number"
                            id="firstName"
                            defaultValue="1"
                            min={"1"}
                            max={"10000"}
                            autoComplete={"off"}
                            autoFocus={true}
                            autoCapitalize={"off"}
                            hidden={false }
                            onChange={(e) => {
                                const val = Number(e.target.value);
                                if (val < 1) e.target.value = "1";
                                else if (val > 10000) e.target.value = "10000";
                                else {
                                    setRunTasks(val);
                                }
                            }}
                        />
                    </div>

                </Flex>

                <Flex direction="row" gap="3" justify="center" align="center">
                    <Button color="cyan"
                            style={{margin: "10px", padding: "10px", width: "114px", borderRadius: "10px"}} size="4"
                            variant="soft" onClick={() => {
                        if (props.onStartRecording) {
                            props.onStartRecording()
                        }
                        setStartToast(true)
                    }}>
                        开始录制
                    </Button>
                    <Toaster
                        position="top-center"
                        reverseOrder={false}
                    />

                    <Button color="pink"
                            style={{margin: "10px", padding: "10px", width: "114px", borderRadius: "10px"}}
                            size-='4' variant="soft" onClick={(_) => {
                        if (props.onStopRecording) {
                            props.onStopRecording();
                        }
                        ShowStopToast();
                    }}>
                        结束录制
                    </Button>
                    <StartExecuteRecordTask ClickEvent={clickEvent}/>
                    <Box onClick={setExecuteStatus}>
                        <Button color="violet"
                                style={{margin: "10px", padding: "10px", width: "114px", borderRadius: "10px"}} size='4'
                                variant="solid">
                            结束执行
                        </Button>
                    </Box>

                    <Button color="indigo"
                            style={{margin: "10px", padding: "10px", width: "114px", borderRadius: "10px"}}
                            size='4'
                            variant="classic" onClick={() => {
                        if (props.ClearEvent) {
                            props.ClearEvent();
                        }
                        clearToast();
                    }}>

                        清空事件
                    </Button>
                </Flex>

            </Container>
        </Box>

    );

}

