import {invoke} from "@tauri-apps/api/core";

export function writeToLogFile(...message: string[]): void {
    if (process.env.NODE_ENV === 'development') {
        message.forEach(message => {
            //Tauri 会自动提取 序列化后的 message 字段, 根据调用函数的参数名进行序列化
            invoke('write_to_log_file', {
                message: message,
            }).then(_r => {
            })
        })

    }
}