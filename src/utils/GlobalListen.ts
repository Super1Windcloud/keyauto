import {invoke} from "@tauri-apps/api/core";
import {taskRunningToast} from "@/components/Toast.tsx";


export function globalListenKeyDown(setProcessId: (id: number) => void) {

    console.log("Global ListenKeyDown Started");
    invoke("global_listen_key_down").then((pid) => {
        console.log("Global ListenKeyDown Started with pid: " + pid);
        if (typeof pid === "number" && pid > 0) {
            setProcessId(pid);
        }
    })
}


export function globalStopListenKeyDown(pid: number, setProcessId: (id: number) => void) {
    console.log("Global ListenKeyDown Stopped");
    if (pid === 0 || !pid) {
        console.error("Global ListenKeyDown Stop Failed, pid is " + pid)
        return;
    }
    invoke("global_stop_listen_key_down", {pid}).then((
        _r
    ) => {
        if (_r === false) {
            console.error("Global ListenKeyDown Stop Failed")
        } else {
            setProcessId(0);
            console.log("listenkey stopped with result type :" + typeof _r)
        }
    })
}

export function readRecordFile(emptyRecordFileToast: () => void, count: number) {
    invoke("read_record_key_from_file",).then((content) => {
        if (!content) {
            emptyRecordFileToast();
            return;
        } else {
            let HigherOrderCallback = (count: number) => {
                return () => invokeExecuteTask(count)
            }
            taskRunningToast(HigherOrderCallback(count));
        }
    })
}


export async  function invokeExecuteTask(count: number) {
    return invoke("execute_record_key_file", {count}).then(_r => {
        if (typeof _r === "string" && !_r) {
            console.error("execute_record_key_file result is empty, please try record")
            return false
        } else {
            return true
        }
    });
}

export function globalListenLeftClick() {
    invoke("global_listen_left_click").then(() => {
    })
}

export function globalListenRightClick() {
    invoke("global_listen_right_click").then(() => {
    })
}


