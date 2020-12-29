import { createApp } from 'vue';
import CreatePost from './components/CreatePost.vue';
import Messages from './components/Messages.vue';

createApp(CreatePost).mount("#createpost-container")
createApp(Messages).mount("#messages-app")
