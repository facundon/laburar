<script lang="ts">
	import { check } from '@tauri-apps/plugin-updater'
	import { onDestroy, onMount } from 'svelte'

	let status = $state<string | null>(null)
	let total = $state(0)
	let downloaded = $state(0)
	let percentage = $derived(((downloaded / total) * 100).toFixed(2))

	function formatBytes(bytes: number) {
		const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
		if (bytes === 0) return '0 Byte'
		const i = Math.floor(Math.log(bytes) / Math.log(1024))
		return parseFloat((bytes / Math.pow(1024, i)).toFixed(2)) + ' ' + sizes[i]
	}

	async function checkUpdate() {
		console.log('checking for updates')
		const update = await check()
		if (!update) return

		await update.downloadAndInstall(event => {
			switch (event.event) {
				case 'Started':
					total = event.data.contentLength || 0
					status = `Descargando ${update.version}...`
					break
				case 'Progress':
					downloaded += event.data.chunkLength
					break
				case 'Finished':
					status = 'Instalando...'
					break
			}
		})
		status = null
	}

	let interval = $state<number | null>(null)
	onMount(() => {
		checkUpdate()
		interval = setInterval(checkUpdate, 1000 * 60)
	})
	onDestroy(() => {
		if (interval) clearInterval(interval)
	})
</script>

{#if status}
	<div class="update-bar">
		{#if total > 0}
			<p>
				{status}{' '}
				<span>{percentage}% ({formatBytes(downloaded)} / {formatBytes(total)})</span>
			</p>
		{/if}
		<div class="progress" style="width: {percentage}%"></div>
	</div>
{/if}

<style>
	.update-bar {
		margin: auto;
		background-color: var(--gray-light);
		border-bottom-left-radius: 8px;
		border-bottom-right-radius: 8px;
		text-align: center;
		width: 90%;
		position: relative;
		z-index: 1;
	}

	p {
		font-size: 0.8rem;
		gap: 2rem;
		color: white;
		padding-block: 0.25rem;
		margin: 0;
	}

	p > span {
		font-weight: 600;
	}

	.progress {
		height: 100%;
		background: linear-gradient(270deg, rgb(18, 129, 79), #2e949b, #5e94ac, #aa44c4);
		background-size: 400% 400%;
		animation: gradientAnimation 5s ease infinite;
		border-bottom-left-radius: 8px;
		border-bottom-right-radius: 8px;
		position: absolute;
		top: 0;
		left: 0;
		z-index: -1;
		transition: width 0.5s ease;
	}

	@keyframes gradientAnimation {
		0% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
		100% {
			background-position: 0% 50%;
		}
	}
</style>
