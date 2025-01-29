<script lang="ts">
    import '../app.css'
    import { Toaster } from '$lib/components/ui/sonner'
    import { Button } from '$lib/components/ui/button'

    import { goto } from '$app/navigation'

    let { children } = $props()
    $effect(() => {
        document.documentElement.classList.add('dark')
    })
</script>

<main class="min-h-screen bg-background text-foreground">
    <Button onclick={() => goto('/')}>.home</Button>
    <Button onclick={() => goto('/recording')}>.recording</Button>
    {@render children()}
</main>

<Toaster theme="dark" />
