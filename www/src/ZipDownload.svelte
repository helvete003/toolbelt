<script>
    import {
        Icon,
        Row,
        Col,
        ListGroupItem
    } from 'sveltestrap';
    export let zip_file_url;
</script>

<ListGroupItem color="dark">
    <Row>
        <Col md="8" lg="10" class="my-2">
            
        </Col>
        <Col md="4" lg="2">
            <a class="btn btn-primary" href="{zip_file_url}" style="width:100%;" download="converted.zip">Zip</a>
        </Col>
    </Row>
</ListGroupItem>