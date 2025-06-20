import {AlertDialog} from "radix-ui";
import {useExecutionStatus, useRecordStore} from "@/store";
import {useState} from "react";

const ExecuteRecordAlertDialog = (click: () => void) => {

    const isRecord = useRecordStore(state => state.isRecording);
    const [open, setOpen] = useState(false);
    const setExecute = useExecutionStatus(state => state.setIsExecuting)
    const executeStatus = useExecutionStatus(state => state.isExecuting)
    const handleClick = () => {
        if (isRecord) {
            setOpen(true);
        } else {
            if (typeof click === "function") {
                click();
                if (!executeStatus) {
                    setExecute(true);
                }
            } else {
                alert("未知click类型" + typeof click);
            }
        }
    };

    return (<>
        <AlertDialog.Root open={open} onOpenChange={setOpen}>
            <button onClick={handleClick}
                    className="inline-flex h-[35px]
             mx-2.5 p-2.5 rounded-x
              hover:bg-indigo-600
              transition-colors
                duration-200
                w-1/7
                h-[44px]
               items-center justify-center rounded bg-violet4 px-[15px] font-medium leading-none text-violet11 outline-none outline-offset-1 hover:bg-mauve3 focus-visible:outline-2 focus-visible:outline-violet6 select-none">
                启动执行
            </button>
            <AlertDialog.Portal>
                <AlertDialog.Overlay className="fixed inset-0 bg-black/30   data-[state=open]:animate-overlayShow"/>
                <AlertDialog.Content
                    className="fixed left-1/2 top-1/2 max-h-[85vh] w-[90vw] max-w-[500px] -translate-x-1/2 -translate-y-1/2
                     rounded-md bg-black  p-[25px] shadow-[var(--shadow-6)] focus:outline-none data-[state=open]:animate-contentShow">
                    <AlertDialog.Title className="m-0 text-[17px] font-medium text-pink-500 ">
                        启动前请结束录制
                    </AlertDialog.Title>
                    <AlertDialog.Description/>
                    <div className="flex justify-end gap-[25px]">
                        <AlertDialog.Action asChild>
                            <button
                                onClick={() => setOpen(false)}
                                className="inline-flex h-[35px] items-center justify-center rounded bg-red4 px-[15px] font-medium leading-none text-red11 outline-none outline-offset-1 hover:bg-red5 focus-visible:outline-2 focus-visible:outline-red7 select-none">
                                确定
                            </button>
                        </AlertDialog.Action>
                    </div>
                </AlertDialog.Content>
            </AlertDialog.Portal>

        </AlertDialog.Root>
    </>);
}


export function StartExecuteRecordTask(props : { ClickEvent: () => void }  ) {
    return ExecuteRecordAlertDialog(props.ClickEvent);
}