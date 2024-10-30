<template>
<div>

</div>
</template>

<script setup lang="ts">
import { GroupService } from '~/services/group.service';
import type { Profile } from '~/types/profile.type';

const route = useRoute();
const { code } = route.params;

const supabaseUser = useSupabaseUser();
const { $localApi } = useNuxtApp();
const groupService = new GroupService($localApi);




</script>