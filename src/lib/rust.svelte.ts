import { invoke } from '@tauri-apps/api/core'

export const preventDefault = <T extends Event>(
    fn: (e: T) => void
): ((e: T) => void) => {
    return (e: T) => {
        e.preventDefault()
        fn(e)
    }
}

export class GlobalState {
    private _state = $state({ isRecording: false })

    set isRecording(value: boolean) {
        this._state.isRecording = value
    }

    get isRecording() {
        return this._state.isRecording
    }

    async toggleRecord() {
        this.isRecording = await invoke('record', {
            isRecording: this.isRecording,
        })
    }
}
