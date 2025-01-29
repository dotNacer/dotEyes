<script lang="ts">
    import { Button } from '$lib/components/ui/button'
    import { GlobalState } from '$lib/rust.svelte'
    import { onMount } from 'svelte'
    import { toast } from 'svelte-sonner'

    const gs = new GlobalState()

    onMount(() => {
        gs.checkStatus()
    })

    async function handleToggleRecording() {
        await gs.toggleRecord()
    }
</script>

<div class="container mx-auto p-4">
    <h1 class="text-2xl font-bold mb-4">Screen Recording</h1>

    <div class="space-y-4">
        {#if gs.error}
            <!-- METTRE UN TOAST -->
        {/if}

        <div class="flex items-center space-x-4">
            <div class="flex-1">
                <p class="text-lg">
                    Status: <span class="font-bold"
                        >{gs.isRecording ? 'Recording' : 'Stopped'}</span
                    >
                </p>
            </div>

            <Button
                variant={gs.isRecording ? 'destructive' : 'default'}
                onclick={handleToggleRecording}
            >
                {gs.isRecording ? 'Stop Recording' : 'Start Recording'}
            </Button>
        </div>
    </div>
</div>
