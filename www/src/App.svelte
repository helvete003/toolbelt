<script>
	import { 
		Alert,
		Col,
		Row,
		ListGroup,
		InputGroup,
		ButtonDropdown,
		DropdownItem,
		DropdownMenu,
		DropdownToggle
	} from 'sveltestrap';
	import ListElement from './List-Element.svelte';
	import Promo from './Promo.svelte';
	import Zip from './ZipDownload.svelte';
	import { onMount } from 'svelte';
	import JSZip from 'jszip';

	let file_elements = null;
	let selected_file_count = 0;
	let convert_button = null;
	const converter_worker = new Worker('./converter_worker.js');
	let file_input; //the file input field

	let converted_elements = new Array();
	let zip_file = null;
	let zip_url;
	//Alerts in the system
	let converting_alert = false;
	let zipping_alert = false;
	let error_alert = false;
	let error_text;
	if(!window.Worker) {
		console.log("No webworker support in this browser!");
	}
	onMount(() => {
		convert_button = document.getElementById('convert-button');
		convert_button.disabled = true;

		document.addEventListener("change", function(e) {
			if(e.target && e.target.id== 'file-input') {
				//converter_worker.postMessage(e.target.files);
				//console.log(e.target.files);
				file_elements = e.target.files;
				selected_file_count = file_elements.length;
				convert_button.disabled = false;
			}
		}, false);
		converter_worker.onmessage = function(e) {
			if(e.data.status == "image_done") {
				//let image_blob = new Blob([e.data.converted_image_bytes], { type: e.data.new_type});
				let image_url = URL.createObjectURL(
						new Blob([e.data.converted_image_bytes], { type: e.data.new_type})
						);
				converted_elements = [...converted_elements, {image_url, new_name: e.data.new_name}]; //Reactive assigment of arrays
				zip_file.file(e.data.new_name, e.data.converted_image_bytes);
				//all images have been converted
			} else if(e.data.status == "image_error") {
				error_alert = true;
				error_text = e.data.error_message;
				--selected_file_count; //we remove some images from the counter
			}
			if(converted_elements.length == selected_file_count) {
				file_input.value = "";
				convert_button.disabled = true;
				converting_alert = false;
				if(converted_elements.length > 0) {
					zipping_alert = true;
					zip_file.generateAsync({type:"base64"}).then(function (base64) {
						zip_url = "data:application/zip;base64," + base64;
						zipping_alert = false;
					}, function (err) {
						error_alert = true;
						error_text = err.message;
					});
				}
			}
		};
	});
	/**
	* The on click function to start the coversion.
	*
	* @param {string} intoFormat
	* @returns {Uint8Array}
	*/
	function startConvert(into_format) {
		converting_alert = true;
		zip_url = undefined;
		zip_file = new JSZip();
		error_alert = false;
		error_text = "";
		converted_elements = new Array();
		//console.log(convert_button);
		convert_button.disabled = true;
		converter_worker.postMessage({file_elements, into_format});

	}
</script>

<Row class="headerbelt">
	<Col xs="12" md="3" lg="2"><h1 id="logo">toolbelt</h1></Col>
	<Col xs="12" md="9" lg="10">
		<InputGroup id="toolbelt">
			<input class="form-control" type="file" id="file-input" accept=".jpg, .png, .jpeg, .gif, .bmp, .tif, .tiff|image/*" multiple bind:this={file_input}>
			<ButtonDropdown>
				<DropdownToggle id="convert-button" color="primary" caret>
					Convert to
				</DropdownToggle>
				<DropdownMenu>
				  <DropdownItem on:click={() => startConvert("Gif")}>Gif</DropdownItem>
				  <DropdownItem on:click={() => startConvert("Jpeg")}>Jpeg</DropdownItem>
				  <DropdownItem on:click={() => startConvert("Png")}>Png</DropdownItem>
				  <DropdownItem on:click={() => startConvert("Ico")}>Ico</DropdownItem>
				  <DropdownItem on:click={() => startConvert("Tga")}>Tga</DropdownItem>
				  <DropdownItem on:click={() => startConvert("Bmp")}>Bmp</DropdownItem>
				</DropdownMenu>
			  </ButtonDropdown>
		</InputGroup>
	</Col>
</Row>
<Promo  />
<Row class="my-3">
	<Col>
		<Alert color="warning" isOpen={converting_alert}>
			<Row>
				<Col>
					<span class="align-middle">Converting images...</span>
				</Col>
				<Col class="col-1">
					<div class="spinner-border" role="status" style="float:right;"></div> 
				</Col>
			</Row>
		</Alert>
		<Alert color="warning" isOpen={zipping_alert}>
			<Row>
				<Col>
					<span class="align-middle">Zipping images...</span>
				</Col>
				<Col class="col-1">
					<div class="spinner-border" role="status" style="float:right;"></div> 
				</Col>
			</Row>
		</Alert>
		<Alert color="danger" isOpen={error_alert}>
			<Row>
				<Col>
					<span class="align-middle">{error_text}</span>
				</Col>
				<Col class="col-1">
				</Col>
			</Row>
		</Alert>
		<div id="image-container" >
			<ListGroup>
				{#if zip_url != undefined}
					<Zip zip_file_url={zip_url}/>
				{/if}
				{#each converted_elements as element}
					<ListElement file_element={element} />
				{/each}
			</ListGroup>
		</div>
	</Col>
</Row>