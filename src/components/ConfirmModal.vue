<script setup lang="ts">
interface Props {
  show: boolean
  message: string
}

const props = defineProps<Props>()
</script>

<template>
  <teleport to="body">
    <transition name="modal">
      <div v-if="props.show">
        <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"/>
        <div class="fixed z-10 inset-0 overflow-y-auto">
          <div class="flex items-center justify-center min-h-full">
            <div class="bg-white p-6 rounded">
              <div class="pb-4">
                {{ props.message }}
              </div>
              <div>
                <slot/>
              </div>
            </div>
          </div>
        </div>
      </div>
    </transition>
  </teleport>
</template>

<style scoped>
.modal-enter-active {
  transition: opacity 0.2s ease-in;
}
.modal-leave-active {
  transition: opacity 0.2s ease-out;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>
