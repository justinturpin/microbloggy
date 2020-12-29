<template>
    <div id="error-messages" v-if="messages.bad.length > 0">
        <ul v-for="message in messages.bad">
            <li>{{ message }}</li>
        </ul>
    </div>

    <div id="good-messages" v-if="messages.good.length > 0">
        <ul v-for="message in messages.good">
            <li>{{ message }}</li>
        </ul>
    </div>
</template>

<script>
import messages from "../messages";

export default {
    data() {
        return {
            messages: messages
        }
    },
}
</script>
