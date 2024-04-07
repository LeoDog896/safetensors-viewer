<script lang="ts">
	import { onMount } from 'svelte';
    import init, { deserialize } from 'viewer-wasm';

    onMount(async () => {
        await init();
    });

    let input: HTMLInputElement;

    function onChange() {
        if (input.files === null) return;
        if (input.files.length === 0) return;
        const file = input.files[0];
        const reader = new FileReader();

        reader.onload = async () => {
            const buffer = reader.result as ArrayBuffer;
            const data = new Uint8Array(buffer);

            console.log(deserialize(data));
        };

        reader.readAsArrayBuffer(file);
    }
</script>

<input
    type="file"
    accept=".safetensors"
    bind:this={input}
    on:change={onChange}
/>
