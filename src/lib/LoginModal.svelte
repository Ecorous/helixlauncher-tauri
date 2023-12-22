<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { appWindow } from '@tauri-apps/api/window';

    let message: string;
    message = "Loading";
    appWindow.listen("login-phase", (e) => {
        console.log("im here :::");
        console.log(e)
        // @ts-ignore
        message = e.payload.message;
        message = message.replaceAll("https://www.microsoft.com/link", "<a target=\"_blank\" href=\"https://www.microsoft.com/link\">https://www.microsoft.com/link</a>")
    });
    appWindow.listen("auth-finished", (e) => {
        message = e.payload as string
    })
    onMount(async () => {
        await invoke("login_phase_one");
    });
</script>

<!-- svelte-ignore a11y-missing-content -->
<h1 bind:innerHTML={message} contenteditable="false"></h1>
