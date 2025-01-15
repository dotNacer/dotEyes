<script lang="ts">
    import { GlobalState, preventDefault } from '$lib'
    import { Button } from '$lib/components/ui/button/index.js'
    import { Input } from '$lib/components/ui/input/index.js'
    import * as Card from '$lib/components/ui/card/index.js'
    import { fade } from 'svelte/transition'

    const gs = new GlobalState()

    $inspect(gs.greet, gs.name)

    const handleSubmit = () => {
        gs.nlen && gs.submit()
    }

    const handleReset = () => {
        gs.reset()
    }
</script>

<div
    class="min-h-screen flex items-center justify-center p-4 transition-all duration-500"
>
    <Card.Root
        class="w-[380px] backdrop-blur-sm bg-background  transition-all duration-300  [--ring:267_100%_60%]"
    >
        <Card.Header class="space-y-2">
            <Card.Title class="text-3xl font-bold text-center text-foreground">
                {#if gs.greet}
                    <div transition:fade>
                        <p>{gs.greet}</p>
                        <small class="text-sm">(from Rust side)</small>
                    </div>
                {:else}
                    <p>Hello World!</p>
                {/if}
            </Card.Title>
        </Card.Header>
        <Card.Content class="p-6">
            <form onsubmit={handleSubmit} class="space-y-6">
                {#if !gs.greet}
                    <Input
                        type="text"
                        placeholder="Enter your username"
                        bind:value={gs.name}
                        class="w-full px-4 py-2  outline-none border border-border transition-all duration-200"
                    />
                {/if}
                {#if gs.name && !gs.greet}
                    <Button
                        type="submit"
                        class="w-full  hover:opacity-90 transition-opacity duration-200"
                    >
                        Say Hello
                    </Button>
                {:else if gs.greet}
                    <Button
                        onclick={handleReset}
                        class="w-full hover:opacity-90 transition-opacity duration-200"
                        >Reset</Button
                    >
                {/if}
            </form>
        </Card.Content>
    </Card.Root>
</div>
