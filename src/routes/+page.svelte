<script lang="ts">
	import { onMount } from 'svelte';
    import init, { deserialize } from 'viewer-wasm';
    import { z } from 'zod';

    const deserializeSchema = z.object({
        tensors: z.array(
            z.tuple([
                z.string(),
                z.object({
                    data: z.array(z.number()),
                    dtype: z.string(),
                    shape: z.array(z.number()),
                })
            ])
        ),
    });

    type DeserializeData = z.infer<typeof deserializeSchema>;

    onMount(async () => {
        await init();
    });

    let input: HTMLInputElement;

    let data: DeserializeData | undefined | [error: string] = undefined;

    function onChange() {
        if (input.files === null) return;
        if (input.files.length === 0) return;
        const file = input.files[0];
        const reader = new FileReader();

        reader.onload = async () => {
            const buffer = reader.result as ArrayBuffer;
            const bufferedData = new Uint8Array(buffer);

            const parsedData = deserializeSchema.safeParse(deserialize(bufferedData));

            if (parsedData.success) {
                data = parsedData.data;
            } else {
                data = [parsedData.error.errors[0].message];
            }
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

{#if data === undefined}
    <p>Upload a file to get started.</p>
{:else}
    {#if Array.isArray(data)}
        <p>{data[0]}</p>
    {:else}
        <pre>{JSON.stringify(data, null, 2)}</pre>
    {/if}
{/if}
