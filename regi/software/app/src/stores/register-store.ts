import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import { RegisterType } from '../schema/register'
import { CombinedRegistersType } from '../schema/combined-register'
import { MockType } from '../schema/mock'
import { InterpreterType } from '../schema/interpreter'
import { TranslatorType } from '../schema/translator'

interface RegisterState {
  registers: RegisterType[]
  combinedRegister: CombinedRegistersType[]
  mocks: MockType[]
  interpreters: InterpreterType[]
  translators: TranslatorType[]
}

interface RegisterStore extends RegisterState {
  createNewRegister: () => void
}

const initialState: RegisterState = {
  registers: [],
  combinedRegister: [],
  mocks: [],
  interpreters: [],
  translators: []
}

export const useRegisterStore = create<RegisterStore>()(
  immer((set, get) => ({
    ...initialState,

    createNewRegister: () => { }

  }))
)
