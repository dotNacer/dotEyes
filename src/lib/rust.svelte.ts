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
    private _state = $state({
        isRecording: false,
        error: null as string | null,
    })

    set isRecording(value: boolean) {
        this._state.isRecording = value
    }

    get isRecording() {
        return this._state.isRecording
    }

    get error() {
        return this._state.error
    }

    async toggleRecord() {
        try {
            this.isRecording = await invoke('record', {
                isRecording: this.isRecording,
            })
            this._state.error = null
        } catch (err) {
            this._state.error = err.message
        }
    }

    async checkStatus() {
        this.isRecording = await invoke('get_recording_status')
    }
}
