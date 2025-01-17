<script lang="ts">
    import '../app.css'
    import { Toaster } from 'svelte-sonner'

    $effect(() => {
        document.documentElement.classList.add('dark')
    })
</script>

<main class="min-h-screen bg-background text-foreground">
    <slot />
</main>

<Toaster theme="dark" />
