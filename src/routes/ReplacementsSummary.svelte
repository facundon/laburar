<script lang="ts">
	import Button from '$components/Button.svelte'
	import SummaryContainer from '$components/SummaryContainer.svelte'
	import { Replacement } from '$models/replacement.svelte'
	import { ROUTES } from '$routes'
	import { formatDateToFullDay } from '$utils'
	import { isToday } from 'date-fns'
	import { Repeat } from 'lucide-svelte'
	import Confetti from 'svelte-confetti'

	interface Props {
		replacements: Replacement[]
	}

	let { replacements }: Props = $props()

	let todayReplacements = $state<Replacement[]>([])
	let futureReplacements = $state<Replacement[]>([])

	$effect(() => {
		todayReplacements = replacements.filter(replacement => isToday(replacement.replacementStartDate))
		futureReplacements = replacements.filter(replacement => !isToday(replacement.replacementStartDate))
	})
</script>

{#snippet Action()}
	<Button variant="secondary" outlined href={ROUTES.replacement.list} Icon={Repeat}>Ver Reemplazos</Button>
{/snippet}

<SummaryContainer title="Reemplazos 🔁" {Action}>
	{#if replacements.length === 0}
		<p class="empty">
			No hay nadie reemplazando 🎉
			<Confetti x={[0.5, 3]} />
		</p>
	{/if}
	{#if todayReplacements.length > 0}
		<h3>Ahora</h3>
		{#each todayReplacements as replacement}
			<p>
				{replacement.originalEmployeeName} <span>→</span>
				{replacement.replacementEmployeeName} (hasta el
				<span>{formatDateToFullDay(replacement.replacementEndDate, true)}</span>)
			</p>
		{/each}
	{/if}
	{#if futureReplacements.length > 0}
		<h3>Próximamente</h3>
		{#each futureReplacements as replacement}
			<p>
				{replacement.originalEmployeeName} <span>→</span>
				{replacement.replacementEmployeeName} (desde el <span>{formatDateToFullDay(replacement.replacementStartDate, true)}</span> hasta el
				<span>{formatDateToFullDay(replacement.replacementEndDate, true)}</span>)
			</p>
		{/each}
	{/if}
</SummaryContainer>
