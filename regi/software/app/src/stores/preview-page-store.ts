import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import { RegisterType } from '../schema/register'
import { CombinedRegistersType } from '../schema/combined-register'
import { MockType } from '../schema/mock'
import { BitConditionType, InterpreterRegisterBitsType, InterpreterType, InterpreterTypeEnum } from '../schema/interpreter'
import { eightEmptyBits } from '../empty-objects/register'
import { BitType } from '../schema/bits'
import { move } from '@dnd-kit/helpers'

interface IPreviewPageState {
  registers: RegisterType[]
  combinedRegister: CombinedRegistersType[]
  interpreters: InterpreterType[]
  mocks: MockType[]
}

interface IPreviewPageStore extends IPreviewPageState {
}

const initialState: IPreviewPageState = {
  registers: [],
  combinedRegister: [],
  mocks: [],
  interpreters: [],
}

export const usePreviewPageStore = create<IPreviewPageStore>()(
  immer((set, get) => ({
    ...initialState,

  }))
)
