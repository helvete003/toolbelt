<script>
    import {
        Icon,
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
    <a href="//github.com/helvete003/toolbelt" class="modal_link" target="_blank">
        <Icon name="github" ></Icon>
    </a>
</div>