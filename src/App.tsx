import "@radix-ui/themes/styles.css";
import {KeyAutoTask} from "./KeyAutoTask";
import {ThemeProvider} from "next-themes";
import {invoke} from "@tauri-apps/api/core";
import {useEffect} from "react";
import './App.css'
import {writeToLogFile} from "@/utils/logger.ts";

function App() {
    useEffect(() => {
        writeToLogFile("调用show_window")
        setTimeout(async () => {
            await invoke("show_window")
        }, 50);
    }, [])
    writeToLogFile("渲染App.tsx")
    return (
        <ThemeProvider attribute="class">
            <KeyAutoTask/>
        </ThemeProvider>
    );
}

export default App;
