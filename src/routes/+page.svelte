<script lang="ts">
	import Button from '$components/Button.svelte'
	import CongratsText from '$components/CongratsText.svelte'
	import { invoke } from '$invoke'
	import Confetti from 'svelte-confetti'
</script>

<main>
	<h1><CongratsText>Bienvenida !!!</CongratsText></h1>
	<div class="full-screen-confetti">
		<Confetti x={[-3, 5]} y={[0, 0.3]} delay={[100, 1000]} infinite duration={3000} amount={500} fallDistance="100vh" />
	</div>

	<Button onclick={async () => invoke('sugest_employees_for_assignation_command', { assignment_id: 2 }).then(console.log)}>Computar</Button>
</main>

<style>
	h1 {
		text-align: center;
		font-size: 5rem;
		margin-top: 2rem;
	}

	main {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 80vh;
	}

	.full-screen-confetti {
		position: fixed;
		top: -50px;
		left: 0;
		height: 100vh;
		width: 100vw;
		display: flex;
		justify-content: center;
		overflow: hidden;
		pointer-events: none;
	}
</style>
