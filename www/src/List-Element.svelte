<script>
    import {
        Icon,
        Row,
        Col,
        ListGroupItem
    } from 'sveltestrap';

    export let file_element;
</script>

<ListGroupItem>
    <Row>
        <Col md="8" lg="10" class="my-2">
            {file_element.new_name}
        </Col>
        <Col md="4" lg="2">
            <a class="btn btn-primary" href="{file_element.image_url}" download="{file_element.new_name}" style="width:100%;">Download</a>
        </Col>
    </Row>
</ListGroupItem>