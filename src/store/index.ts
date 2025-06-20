import {create} from 'zustand';
import {invoke} from "@tauri-apps/api/core";

interface RecordState {
    isRecording: boolean;
    setIsRecording: (val: boolean) => void;
}

export const useRecordStore = create<RecordState>((set) => ({
    isRecording: false,
    setIsRecording: (val) => set({isRecording: val}),
}));


export  type EventInfo = {
    type: "keydown" | "contextmenu" | "mousedown" | "keyup" | "wheel";
    detail: string;
    timestamp: string;
};
export type KeyEvent = {
    type: "keydown";
    key: string;
    timestamp: string;
};

export type ClickEvent = {
    type: "click" | "right-click";
    x: number;
    y: number;
    timestamp: string;
};

export  type MouseMoveEvent = {
    type: "mousemove";
    x: number;
    y: number;
    timestamp: string;
}
export type RecordEvent = KeyEvent | ClickEvent | MouseMoveEvent;


interface RecordEventState {
    events: EventInfo[];
    addEvent: (event: EventInfo) => void;
    clearEvents: () => void;
    ResetEvents: () => void;
    SetEvents: (event: EventInfo[]) => void;
}

export const useRecordEventStore = create<RecordEventState>((set
) => ({
    events: [] as EventInfo[],
    addEvent: (event: EventInfo) => set((state) => ({
        events: [...state.events, event]
    }))
    , SetEvents: (events: EventInfo[]) => {
        set({events: events});
    }
    ,
    clearEvents: () => {
        set({events: []});
        invoke("init_record_key_file").then(() => {

        })
    }
    ,
    ResetEvents: () => {
        set({events: []});
    }
}))


interface ProcessStatus {
    processId: number;
    setProcessId: (id: number) => void;
}

export const useRecordProcessId = create<ProcessStatus>(
    (set) => ({
        processId: 0,
        setProcessId: (id) => set({processId: id})
    })
);


interface ExecutionStatus {
    isExecuting: boolean;
    setIsExecuting: (val: boolean) => void;
}

export const useExecutionStatus = create<ExecutionStatus>(
    (set) => ({
        isExecuting: false,
        setIsExecuting: (val) => set({isExecuting: val})
    })
);


interface RunTaskCounts {
    runTaskCount: number;
    setRunTaskCount: (count: number) => void;
}

export const useRunTaskCount = create<RunTaskCounts>(
    (set) => ({
    runTaskCount: 1,
    setRunTaskCount: (count) => set({runTaskCount: count})
}));