<script>
    /**
     * @typedef {('fabric'|'btw'|'neoforge')} Loader
     */
    /** @type {Loader} */
    export let loader = 'fabric';
    export let compact = false;

    const labelByLoader = {
        fabric: 'Fabric',
        btw: 'BTW',
        neoforge: 'NEO'
    };

    // Brand colors chosen to resemble logos while working in dark/light themes
    // fabric: cloth-like warm tan/orange; btw: industrial gray-red; neo: neon orange
    const stylesByLoader = {
        fabric: {
            bg: 'linear-gradient(135deg, #d6b48a 0%, #a07c4a 100%)',
            color: '#111'
        },
        btw: {
            bg: 'linear-gradient(135deg, #5a5a5a 0%, #8c2f2f 100%)',
            color: '#fff'
        },
        neoforge: {
            bg: 'linear-gradient(135deg, #ff7a18 0%, #af002d 100%)',
            color: '#fff'
        }
    };

    $: style = stylesByLoader[loader] || stylesByLoader.fabric;
</script>

<span class="badge" class:compact style={`background:${style.bg};color:${style.color}`}>
    {labelByLoader[loader] || loader}
</span>

<style>
    .badge {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: 4px 10px;
        border-radius: 999px;
        font-weight: 800;
        letter-spacing: 0.4px;
        text-transform: uppercase;
        font-size: 12px;
        text-shadow: 0 1px 1px rgba(0,0,0,0.25);
        box-shadow: inset 0 0 0 1px rgba(255,255,255,0.15), 0 4px 10px rgba(0,0,0,0.25);
        user-select: none;
    }
    
    .badge.compact { 
        padding: 2px 8px; 
        font-size: 11px; 
    }
</style>

