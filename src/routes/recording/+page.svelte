<script lang="ts">
    import { Button } from '$lib/components/ui/button'
    import { GlobalState } from '$lib/rust.svelte'

    const gs = new GlobalState()

    function handleToggleRecording() {
        gs.toggleRecord()
    }
</script>

<h1>Recording : {gs.isRecording ? 'ON' : 'OFF'}</h1>

<Button onclick={handleToggleRecording}>
    {gs.isRecording ? 'Stop Recording' : 'Start Recording'}
</Button>
