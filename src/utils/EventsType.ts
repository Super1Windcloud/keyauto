


export type RawEventTypeName = 'KeyPress' | 'KeyRelease' | 'ButtonLeft' | 'ButtonRight' | 'MouseWheel';

// 每种事件对应的原始值类型
export type RawEventType =
    | { KeyPress: string }
    | { KeyRelease: string }
    | { ButtonLeft: [number, number] }
    | { ButtonRight: [number, number] }
    | { MouseWheel: [number, number] };


export type RecordEventsArr = {
    event_name: string,
    event_type: RawEventType
}

export  type RecordEventsType = {
    events: RecordEventsArr[],
    run_task_count : number
}
export type RecordType =
    | { type: 'KeyPress'; value: string }
    | { type: 'KeyRelease'; value: string }
    | { type: 'ButtonLeft'; value: [number, number] }
    | { type: 'ButtonRight'; value: [number, number] }
    | { type: 'MouseWheel'; value: [number, number] };


export function convertRawToRecordType(raw: RawEventType): RecordType {
    const type = Object.keys(raw)[0] as keyof typeof raw;
    const value = raw[type];
    return {
        type,
        value
    } as RecordType;
}

