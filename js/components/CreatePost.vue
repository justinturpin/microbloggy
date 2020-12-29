<template>
  <form action="/post/create" method="POST">
    <input type="hidden" name="csrf-token" value="{{ csrf_token }}">
    <textarea name="content" placeholder="What's happening?"></textarea>
    <input id="createpost-button" type="submit" value="Post">
  </form>

  <div v-if="draft_images.length > 0">
    <h4>Draft Image Uploads</h4>

    <div id="attached-image-container">
      <span v-for="image of draft_images">
        <a target="_blank" :href="'/uploads/' + image.full_path">
          <img class="image-thumbnail" :src="'/uploads/' + image.thumbnail_path">
        </a>
      </span>
    </div>
  </div>

  <h4>Attach Images</h4>

  <form action="/post/image-upload" id="image-upload-form" @submit.stop.prevent="uploadDraftImage">
    <input type="file" name="image" accept="image/*" id="image-upload-input" v-on:change="draftUploadChanged">
    <input type="submit" value="Upload Image" id="image-upload-submit">
  </form>
</template>

<script>

import messages from "../messages";

export default {
  data() {
    return {
      draft_images: [],
      items: [{ message: 'Foo' }, { message: 'Bar' }],
      draft_file: null,
      messages: messages
    }
  },

  created() {
    this.refreshDrafts()
  },

  methods: {
    refreshDrafts() {
      fetch("/api/index")
        .then( response => response.json() )
        .then( data => {
          this.draft_images = data.draft_images
        })
        .catch((error) => {
          console.error('Error:', error);
        });
    },

    draftUploadChanged(event) {
      this.draft_file = event.target.files.item(0);

      this.messages.good.push("Image ready to be uploaded!");
    },

    uploadDraftImage() {
      let reader = new FileReader();

      reader.onload = () => {
          let imageData = reader.result;

          let xhr = new XMLHttpRequest();

          // Reload page when the file is done uploading
          xhr.onreadystatechange = () => {
              if (xhr.readyState == 4) {
                this.refreshDrafts()
              }
          }

          xhr.open("PUT", "/post/image-upload");
          xhr.setRequestHeader("Content-Type", "image/jpeg");
          xhr.send(imageData);
      }

      reader.readAsArrayBuffer(this.draft_file);
    }
  }
}
</script>
