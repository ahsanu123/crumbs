import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'

interface IPreviewPageState {
}

interface IPreviewPageStore extends IPreviewPageState {
}

const initialState: IPreviewPageState = {
}

export const usePreviewPageStore = create<IPreviewPageStore>()(
  immer((set, get) => ({
    ...initialState,

  }))
)
