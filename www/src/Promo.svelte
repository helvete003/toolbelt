<script>
    import {
        Col,
        Row,
        Icon,
        ListGroup,
        ListGroupItem
    } from 'sveltestrap';
    import InfoBubble from './bubbles/Info.svelte';
    import TwitterBubble from './bubbles/Twitter.svelte';
    import GithubBubble from './bubbles/Github.svelte';
</script>
<Row class="promo">
    <Col class="col-3">
        <ListGroup class="list-group-horizontal">
            <ListGroupItem class="bubbles">
                <InfoBubble />
            </ListGroupItem>
            <ListGroupItem class="bubbles">
                <GithubBubble />
            </ListGroupItem>
            <ListGroupItem class="bubbles">
                <TwitterBubble />
            </ListGroupItem>
        </ListGroup>
    </Col>
    
    <Col class="col-9">
    </Col>
    
</Row>