import toast from 'react-hot-toast';


export const notifyStartRecord = () =>
    toast('成功开始键盘和鼠标录制!',
        {
            icon: '👏',
            style: {
                borderRadius: '10px',
                background: '#333',
                color: 'lightpink',
            },
        }
    );


export const notifyStopRecord = () =>
    toast.success('成功停止键盘和鼠标录制!',
        {
            icon: '🐉',
            style: {
                borderRadius: '10px',
                background: '#333',
                color: "hotpink",
            },
        }
    );


export const clearToast = () => {
    toast.success('成功重置已录制事件!', {
        style: {
            borderRadius: '10px',
            background: '#333',
            color: '#fff',
        },
    })
}


export const executeToast = () => {
    toast.success('正在后台执行键鼠事件!', {
        icon: '🦀🐦‍🔥🐉',
        style: {
            borderRadius: '10px',
            background: '#333',
            color: "lightpink",
        },
    })
}

export const stopExecuteToast = () => {
    toast.success('已结束后台任务执行!', {
        icon: '🦀🐦‍🔥🐉',
        style: {
            borderRadius: '10px',
            background: '#333',
            color: "darkred",
        },
    })
}


export const emptyRecordFileToast = () => {
    toast.error('录制文件为空!', {
        icon: '🦀‍',
        style: {
            borderRadius: '10px',
            background: '#333',
            color: "darkred",
        },
    })
}


export const taskRunningToast = (invokeExecuteTask: () => Promise<boolean>) => {
    toast.promise(
        invokeExecuteTask(),
        {
            loading: '正在运行键鼠任务...',
            success: <b>运行成功!</b>,
            error: <b>执行失败!</b>,
        }
    ).then(r => {
        console.log(r);
    });

}