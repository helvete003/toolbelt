<script>
    import {
        Icon,
        Modal
    } from 'sveltestrap';
    let open = false;
    const toggle = () => (open = !open);
</script>
<style>
    .modal_link {
        color:var(--main-link-color);
        
    }
</style>

<div class="text-center">
    
    <a href={"#"} class="modal_link" on:click={toggle}>
        <Icon name="info-circle" ></Icon>
    </a>
    <Modal body header="Info" isOpen={open} {toggle}>
        <p>
            toolbelt is a WASM based image converter, which means that the conversion happens in your browser (no files will leave your device). <br>
            Which also means that depending on the device you are using, it can take longer to convert the images. 
        </p>
        <p>
            Version: 0.2 beta
        </p>
    </Modal>
</div>