<script lang="ts" generics="T extends string | number ">
	import { ChevronDown, Eraser } from 'lucide-svelte'
	import type { HTMLSelectAttributes } from 'svelte/elements'
	import { onDestroy, onMount } from 'svelte'

	interface Props<T> extends Omit<HTMLSelectAttributes, 'onchange' | 'value'> {
		value?: T | null
		onchange?: (value: T | null) => void
	}

	let { children, value = $bindable(), onchange, disabled = false, ...rest }: Props<T> = $props()

	let searchInput: HTMLInputElement
	let wrapper: HTMLDivElement
	let select: HTMLSelectElement
	let options: HTMLOptionElement[] = $state([])
	let search = $state('')
	let isOpen = $state(false)
	let highlightedIndex = $state(-1)

	function handleButtonClick(e: Event) {
		if (disabled) return
		isOpen = !isOpen
		if (isOpen) searchInput?.focus()
	}

	function handleInputFocus(e: Event) {
		if (disabled) return
		isOpen = true
		filterOptions()
	}

	function filterOptions() {
		options = Array.from(select.options).filter(option => option.text.toLowerCase().includes(search.toLowerCase()))
		highlightedIndex = 0
	}

	function handleKeydown(event: KeyboardEvent) {
		if (!isOpen) return

		if (event.key === 'ArrowDown') {
			event.preventDefault()
			highlightedIndex = (highlightedIndex + 1) % options.length
		} else if (event.key === 'ArrowUp') {
			event.preventDefault()
			highlightedIndex = (highlightedIndex - 1 + options.length) % options.length
		} else if (event.key === 'Enter' && highlightedIndex >= 0) {
			event.preventDefault()
			setValue(options[highlightedIndex])
		} else if (event.key === 'Escape') {
			isOpen = false
		}
	}

	const parseNumber = (value: string) => {
		const parsed = parseInt(value, 10)
		return isNaN(parsed) ? value : parsed
	}

	function setValue(option: HTMLOptionElement) {
		const newValue = parseNumber(option.value)
		value = newValue as T
		search = option.text
		isOpen = false
		onchange?.(newValue as T)
	}

	function clearInput(event: MouseEvent) {
		if (disabled) return
		event.stopPropagation()
		isOpen = true
		search = ''
		filterOptions()
	}

	const handleClickOutside = (event: MouseEvent) => {
		if (wrapper?.contains(event.target as Node)) return
		isOpen = false
	}

	$effect(() => {
		if (isOpen) {
			document.addEventListener('click', handleClickOutside)
		} else {
			document.removeEventListener('click', handleClickOutside)
		}
	})

	$effect(() => {
		if (value) return
		search = ''
	})

	onMount(() => {
		setTimeout(() => {
			const initialValue = select.options[select.selectedIndex]
			if (!initialValue) return
			setValue(initialValue)
		}, 10)
	})

	onDestroy(() => {
		document.removeEventListener('click', handleClickOutside)
	})
</script>

<div class="select-wrapper {disabled ? 'disabled' : ''}" bind:this={wrapper}>
	<div class="input-wrapper">
		<input
			class="search-input"
			bind:this={searchInput}
			bind:value={search}
			oninput={filterOptions}
			onfocus={handleInputFocus}
			onkeydown={handleKeydown}
			placeholder="Buscar..."
			{disabled}
		/>
		{#if search && !disabled}
			<button class="icon clear" onclick={clearInput} type="button" tabindex="-1">
				<Eraser color="var(--error-dark)" strokeWidth={1} />
			</button>
		{/if}
		<button class="icon chevron" onclick={handleButtonClick} type="button" tabindex="-1" {disabled}>
			<ChevronDown color="var(--secondary-dark)" strokeWidth={1} />
		</button>
	</div>

	{#if isOpen}
		<div class="dropdown">
			{#if options.length === 0}
				<div class="dropdown-item disabled" role="option" aria-selected={false} tabindex="0">No hay resultados</div>
			{/if}
			{#each options as option, i}
				<div
					class="dropdown-item {i === highlightedIndex ? 'highlighted' : ''}"
					role="option"
					aria-selected={i === highlightedIndex}
					tabindex="0"
					onmouseenter={() => (highlightedIndex = i)}
					onclick={() => {
						setValue(option)
					}}
					onkeydown={event => {
						if (event.key === 'Enter' || event.key === ' ') {
							setValue(option)
						}
					}}
				>
					{@html option.innerHTML}
				</div>
			{/each}
		</div>
	{/if}

	<select {...rest} bind:this={select} bind:value {disabled}>
		{@render children?.()}
	</select>
</div>

<style>
	.select-wrapper {
		width: fit-content;
		position: relative;
		width: var(--width);
	}
	.input-wrapper {
		display: flex;
		align-items: center;
		position: relative;
	}
	.search-input {
		width: 100%;
		box-sizing: border-box;
		padding: 0.5rem;
	}
	.dropdown {
		position: absolute;
		top: 105%;
		width: 100%;
		z-index: 1;
		background: white;
		border: 1px solid #ccc;
		color: var(--gray-dark);
		max-height: 20rem;
		overflow-y: auto;
	}
	.dropdown-item {
		padding: 0.5rem;
		cursor: pointer;
		display: flex;
		gap: 0.25rem;
		align-items: center;
	}

	.dropdown-item.disabled {
		color: var(--gray-light);
		cursor: not-allowed;
		text-align: center;
	}
	.dropdown-item.highlighted {
		background: var(--primary-light);
	}
	.icon {
		background: transparent;
		border: none;
		position: absolute;
		top: 0.5rem;
		padding-block: 0;
		margin: 0;
		cursor: pointer;
	}

	.chevron {
		right: 0.5rem;
	}

	.clear {
		right: 2.5rem;
	}

	select {
		display: none;
	}

	.select-wrapper.disabled .search-input,
	.select-wrapper.disabled .icon {
		cursor: not-allowed;
	}
</style>
