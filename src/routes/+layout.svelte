<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { AppSidebar, Header } from '$lib/components/layout/index.js';
	import { SidebarInset, SidebarProvider } from '$lib/components/ui/sidebar/index.js';
	import { ModeWatcher } from 'mode-watcher';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<ModeWatcher defaultMode="dark" track={false} />

<SidebarProvider>
	<AppSidebar />
	<SidebarInset>
		<Header />
		<main class="flex-1 overflow-auto p-6">
			{@render children()}
		</main>
	</SidebarInset>
</SidebarProvider>
