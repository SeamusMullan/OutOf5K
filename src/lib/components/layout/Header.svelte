<script lang="ts">
	import { page } from '$app/state';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import { SidebarTrigger } from '$lib/components/ui/sidebar/index.js';

	const titles: Record<string, string> = {
		'/': 'Dashboard',
		'/demos': 'Demos',
		'/stats': 'Statistics',
		'/compare': 'Compare',
		'/settings': 'Settings'
	};

	let pageTitle = $derived(() => {
		const pathname = page.url.pathname;
		if (pathname.startsWith('/demos/')) return 'Demo Viewer';
		return titles[pathname] ?? 'OutOf5K';
	});
</script>

<header class="flex h-12 shrink-0 items-center gap-2 border-b px-4">
	<SidebarTrigger class="-ml-1" />
	<Separator orientation="vertical" class="mr-2 !h-4" />
	<h1 class="text-sm font-medium">{pageTitle()}</h1>
</header>
