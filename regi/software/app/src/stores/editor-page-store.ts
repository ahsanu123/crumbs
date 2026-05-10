import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import { eightEmptyBits } from '../empty-objects/register'
import { BitType } from '../schema/bits'
import { move } from '@dnd-kit/helpers'
import { RegisterSchema } from '../schema/register'
import { CombinedRegisterId, CombinedRegisterSchema } from '../schema/combined-register'
import { InterpreterRegisterBitSchema, InterpreterSchema, InterpreterType } from '../schema/interpreter'
import { MockSchema } from '../schema/mock'
import { orderBy } from 'es-toolkit'

export enum SelectedRegisterType {
  Register,
  CombinedRegister,
}

interface SelectedRegister {
  type: SelectedRegisterType,
  name: string,
  id: number
}

interface IEditorPageState {
  registers: RegisterSchema[]
  selectedRegister: SelectedRegister | undefined,
  selectedInterpreter: InterpreterSchema | undefined,
  combinedRegister: CombinedRegisterSchema[]
  interpreters: InterpreterSchema[]
  mocks: MockSchema[]
}

interface IEditorPageStore extends IEditorPageState {
  addNewRegister: () => RegisterSchema[]

  addNewCombinedRegister: () => CombinedRegisterSchema[]

  updateRegister: (registerId: number, key: 'name' | 'address' | 'description', data: string) => RegisterSchema | undefined,

  updateRegisterBitType: (registerId: number, bitId: number, bitType: BitType) => void

  removeRegister: (registerId: number) => void

  moveCombinedRegister: (combinedId: number, ev: any) => void

  updateRegisterBitResValue: (registerId: number, bitId: number, value: string) => void

  insertRegisterToCombinedRegister: (combinedRegId: number, registerId: number) => void

  removeCombinedRegisterMember: (combinedRegId: number, registerId: number) => void

  removeCombinedRegister: (combinedRegId: number) => void

  reorderCombinedRegister: (combinedRegId: number) => void

  updateCombinedRegister: (combinedId: number, type: 'name' | 'description', data: string) => void

  addNewCombinedRegisterInterpreter: (combinedId: number) => void

  updateInterpreterBits: (interpreterId: number, registerId: number, bitId: number) => void

  removeInterpreter: (interpreterId: number) => void

  updateInterpreterNameOrDescription: (interpreterId: number, type: 'name' | 'description', data: string) => void

  setSelectedRegister: (id: number, type: SelectedRegisterType) => void

  setSelectedInterpreter: (interpreterId: number) => void

  updateInterpreterFormula: (interpreterId: number, formula: string) => void

  updateMockBitValue: (interpreterId: number, bitId: number, value: string) => void

  getMockValue: (interpreterId: number, bitId: number) => string

  getLatestRegisterId: () => number

  getLatestMockId: () => number

  getLatestInterpreterId: () => number

  getLatestCombinedRegisterId: () => number

  getLatestCombinedRegisterOrdinal: (combinedId: number) => number

  getLatestRegisterOrdinal: () => number

  getLatestBitId: (registerId: number) => number | undefined

  getIncludedCombinedRegister: (combinedRegisterId: number) => RegisterSchema[]

  getOrderedCombinedRegisterMember: (combinedRegisterId: number) => RegisterSchema[]

  getCombinedInterpreters: (combinedRegisterId: number) => InterpreterSchema[]

  getInterpreterRegisterBits: (interpreterId: number, registerId: number) => InterpreterRegisterBitSchema | undefined

  getInterpreterBits: (interpreterId: number, registerId: number) => number[]

  isInterpreterRegisterBitIncluded: (interpreterId: number, registerId: number, bitId: number) => boolean
}

const initialState: IEditorPageState = {
  registers: [],
  selectedRegister: undefined,
  selectedInterpreter: undefined,
  combinedRegister: [],
  mocks: [],
  interpreters: [],
}

export const useEditorPageStore = create<IEditorPageStore>()(
  immer((set, get) => ({
    ...initialState,

    addNewRegister: (): RegisterSchema[] => {
      const latestRegisterId = get().getLatestRegisterId()
      const latestRegisterOrdinal = get().getLatestRegisterOrdinal()

      const newRegister: RegisterSchema = {
        register_id: latestRegisterId + 1,
        ordinal: latestRegisterOrdinal + 1,
        name: '',
        description: '',
        address: '',
        bits: [...eightEmptyBits],
      }

      set((state) => {
        state.registers.push(newRegister)
      })

      return get().registers
    },

    moveCombinedRegister: (combinedId: number, ev: any) => {
      set((state) => {
        const combinedRegisters = state.combinedRegister.find(comb => comb.combined_id === combinedId)
        if (!combinedRegisters) return
        const moveResult: CombinedRegisterId[] = move(combinedRegisters.registers as any[], ev)
        const updatedOrdinalResult = moveResult.map((regId, index) => ({
          ...regId,
          ordinal: index
        }))
        combinedRegisters.registers = updatedOrdinalResult
      })
    },

    getOrderedCombinedRegisterMember: (combinedRegisterId: number): RegisterSchema[] => {
      const state = get()
      const combinedRegister = state.combinedRegister.find(combReg => combReg.combined_id === combinedRegisterId)
      if (!combinedRegister) return []

      const registerIdOrdinalMap = new Map(
        combinedRegister.registers.map(combReg => ([combReg.register_id, combReg.ordinal]))
      )

      const registers = state.registers.filter(reg => registerIdOrdinalMap.has(reg.register_id)).map<RegisterSchema>(reg => ({
        ...reg,
        ordinal: registerIdOrdinalMap.get(reg.register_id)!
      }))

      const orderedRegister = orderBy(registers, ['ordinal'], ['asc'])
      return orderedRegister;
    },

    updateRegister: (registerId: number, key: 'name' | 'address' | 'description', data: string) => {
      set((state) => {
        const register = state.registers.find(pr => pr.register_id === registerId)
        if (!register) return;

        if (key === 'name')
          register.name = data

        else if (key === 'description')
          register.description = data

        else if (key === 'address')
          register.address = data

      })
      return get().registers.find(reg => reg.register_id === registerId)
    },

    removeRegister: (registerId: number) => {
      set((state) => {
        state.combinedRegister.forEach(combReg => {
          combReg.registers = combReg.registers.filter(reg => reg.register_id !== registerId)
        })

        state.registers = state.registers.filter(reg => reg.register_id !== registerId)
      })
    },

    updateRegisterBitType: (registerId: number, bitId: number, bitType: BitType) => {
      set((state) => {
        const register = state.registers.find(reg => reg.register_id === registerId)
        if (!register) return;

        const bit = register.bits.find(bit => bit.bit_id === bitId)
        if (!bit) return;

        bit.bit_type = bitType
      })
    },

    updateRegisterBitResValue: (registerId: number, bitId: number, value: string) => {
      set((state) => {
        const register = state.registers.find(reg => reg.register_id === registerId)
        if (!register) return;

        const bit = register.bits.find(bit => bit.bit_id === bitId)
        if (!bit) return;

        bit.reset_val = value
      })
    },

    addNewCombinedRegister: (): CombinedRegisterSchema[] => {
      const state = get()

      const latestCombinedRegisterId = state.getLatestCombinedRegisterId()

      const newCombinedRegister: CombinedRegisterSchema = {
        combined_id: latestCombinedRegisterId + 1,
        registers: [],
        name: `Combined Register ${latestCombinedRegisterId + 1}`
      }

      set((state) => {
        state.combinedRegister.push(newCombinedRegister)
      })

      return get().combinedRegister
    },

    insertRegisterToCombinedRegister: (combinedRegId: number, registerId: number) => {
      set((state) => {
        const combinedRegister = state.combinedRegister.find(comb => comb.combined_id === combinedRegId)
        if (!combinedRegister) return;

        const currentRegistersIds = combinedRegister.registers.map((reg) => reg.register_id)
        const latestCombinedRegisterOrdinal = Math.max(...combinedRegister.registers.map(reg => reg.ordinal))
        if (!currentRegistersIds.includes(registerId)) {
          combinedRegister.registers.push({
            register_id: registerId,
            ordinal: latestCombinedRegisterOrdinal + 1
          })

          const interpreter = state.interpreters.find(int => int.combined_id === combinedRegId && int.type === InterpreterType.CombinedRegister)
          if (!interpreter) return;
          const currentInterpreterRegIds = interpreter.registers.map(reg => reg.register_id)
          if (!currentInterpreterRegIds.includes(registerId))
            interpreter.registers.push({
              register_id: registerId,
              bit_ids: []
            })
        }
      })

      get().reorderCombinedRegister(registerId)
    },

    removeCombinedRegisterMember: (combinedRegId: number, registerId: number) => {
      set((state) => {
        const combinedRegister = state.combinedRegister.find(comb => comb.combined_id === combinedRegId)
        if (!combinedRegister) return;

        const currentRegistersIds = combinedRegister.registers.map((reg) => reg.register_id)
        if (currentRegistersIds.includes(registerId)) {
          combinedRegister.registers = combinedRegister.registers.filter(reg => reg.register_id !== registerId)

          const interpreter = state.interpreters.find(int => int.combined_id === combinedRegId && int.type === InterpreterType.CombinedRegister)
          if (!interpreter) return;
          const currentInterpreterRegIds = interpreter.registers.map(reg => reg.register_id)
          if (!currentInterpreterRegIds.includes(registerId))
            interpreter.registers = interpreter.registers.filter(intReg => intReg.register_id !== registerId)
        }
      })
    },

    removeCombinedRegister: (combinedRegId: number) => {
      set((state) => {
        state.combinedRegister = state.combinedRegister.filter(combReg => combReg.combined_id !== combinedRegId)
        state.interpreters = state.interpreters.filter(int => int.combined_id !== combinedRegId)
      })
    },

    updateCombinedRegister: (combinedRegId: number, type: 'name' | 'description', data: string) => {
      set((state) => {
        const combinedRegister = state.combinedRegister.find(comb => comb.combined_id === combinedRegId)
        if (!combinedRegister) return;
        if (type === 'name')
          combinedRegister.name = data
      })
    },

    reorderCombinedRegister: (combinedRegId: number) => {
      set((state) => {
        const combinedRegister = state.combinedRegister.find(comb => comb.combined_id === combinedRegId)
        if (!combinedRegister) return;

        combinedRegister.registers = combinedRegister.registers
          .slice()
          .sort((a, b) => a.ordinal - b.ordinal)
          .map((reg, index) => ({
            ...reg,
            ordinal: index
          }))

      })
    },

    getLatestRegisterId: (): number => {
      const state = get()

      if (state.registers.length <= 0) return 0
      const lastestRegisterId = Math.max(...state.registers.map((reg) => reg.register_id))

      return lastestRegisterId
    },

    getLatestRegisterOrdinal: (): number => {
      const state = get()

      if (state.registers.length <= 0) return 0
      const latestRegisterOrdinal = Math.max(...state.registers.map((reg) => reg.ordinal))

      return latestRegisterOrdinal
    },

    getLatestMockId: (): number => {
      const state = get()

      if (state.mocks.length <= 0) return 0
      const latestMockId = Math.max(...state.mocks.map((mock) => mock.mock_id))

      return latestMockId
    },

    getLatestInterpreterId: (): number => {
      const state = get()

      if (state.interpreters.length <= 0) return 0
      const latestInterpreterId = Math.max(...state.interpreters.map((int) => int.interpreter_id))

      return latestInterpreterId
    },

    getLatestBitId: (registerId: number): number | undefined => {
      const state = get()
      const register = state.registers.find((pr) => pr.register_id === registerId)

      if (!register) return undefined

      if (register.bits.length <= 0) return 0
      const latestBitsId = Math.max(...register.bits.map((bit) => bit.bit_id))

      return latestBitsId
    },

    getLatestCombinedRegisterId: (): number => {
      const combinedRegister = get().combinedRegister

      if (combinedRegister.length <= 0) return 0
      const latestCombinedRegisterId = Math.max(...combinedRegister.map((comb) => comb.combined_id))
      return latestCombinedRegisterId
    },

    getLatestCombinedRegisterOrdinal: (combinedId: number): number => {
      const combinedRegister = get().combinedRegister.find((comb) => comb.combined_id === combinedId)
      if (!combinedRegister) return 0

      if (combinedRegister.registers.length <= 0) return 0
      const latestCombinedRegisterOrdinal = Math.max(...combinedRegister.registers.map(reg => reg.ordinal))
      return latestCombinedRegisterOrdinal
    },

    addNewCombinedRegisterInterpreter: (combinedId: number) => {
      set((state) => {
        const interpreters = state.interpreters;
        const interpreterId = interpreters.length <= 0 ? 0 : Math.max(...interpreters.map(int => int.interpreter_id))
        const newInterpreterId = interpreterId + 1

        const latestMockId = state.getLatestMockId() + 1

        const combinedRegister = state.combinedRegister.find(combReg => combReg.combined_id === combinedId)
        if (!combinedRegister) return;

        const interpreterRegister: InterpreterRegisterBitSchema[] = []
        combinedRegister.registers.forEach((reg, index) => {
          const newMock: MockSchema = {
            register_id: reg.register_id,
            interpreter_id: newInterpreterId,
            mock_id: latestMockId + index,
            data: []
          }

          interpreterRegister.push({
            register_id: reg.register_id,
            bit_ids: []
          })

          state.mocks.push(newMock)
        })

        const newInterpreter: InterpreterSchema = {
          combined_id: combinedId,
          interpreter_id: newInterpreterId,
          name: `combined register ${combinedId} interpreter ${newInterpreterId}`,
          type: InterpreterType.CombinedRegister,
          description: '',
          registers: interpreterRegister,
          formula: ''
        }
        state.interpreters.push(newInterpreter)
      })
    },

    removeInterpreter: (interpreterId: number) => {
      set((state) => {
        state.interpreters = state.interpreters.filter(int => int.interpreter_id !== interpreterId)
      })
    },

    updateInterpreterBits: (interpreterId: number, registerId: number, bitId: number): void => {
      set((state) => {
        const interpreter = state.interpreters.find(int => int.interpreter_id === interpreterId)
        if (!interpreter) return;

        const register = interpreter.registers.find(reg => reg.register_id === registerId)
        if (!register) return;

        const mock = state.mocks.find(mock => mock.interpreter_id === interpreterId && mock.register_id === registerId)
        if (!mock) return

        const allInterpreterBitsIds = register.bit_ids

        if (!allInterpreterBitsIds.includes(bitId)) {
          register.bit_ids.push(bitId)
          mock.data.push({
            bit_id: bitId,
            value: '0',
          })
        }
        else {
          register.bit_ids = register.bit_ids.filter(bit => bit !== bitId)
          mock.data = mock.data.filter(mock => mock.bit_id !== bitId)
        }
      })
    },

    updateInterpreterFormula: (interpreterId: number, formula: string) => {
      set((state) => {
        const interpreter = state.interpreters.find(int => int.interpreter_id === interpreterId)
        if (!interpreter) return;
        interpreter.formula = formula
      })
    },

    updateInterpreterMock: (interpreterId: number, registerId: number, mock: MockSchema) => {
      set((state) => {

        let stateMockIndex = state.mocks.findIndex(mock => mock.interpreter_id === interpreterId && mock.register_id === registerId)

        if (stateMockIndex === -1) {
          state.mocks.push(mock)
        }
        else {
          state.mocks[stateMockIndex] = mock
        }
      })
    },

    getInterpreterRegisterBits: (interpreterId: number, registerId: number): InterpreterRegisterBitSchema | undefined => {
      const interpreter = get().interpreters.find(int => int.interpreter_id === interpreterId)
      if (!interpreter) return undefined

      const register = interpreter.registers.find(reg => reg.register_id === registerId)
      return register
    },

    getInterpreterBits: (interpreterId: number, registerId: number): number[] => {
      const interpreter = get().interpreters.find(int => int.interpreter_id === interpreterId)
      if (!interpreter) return []

      const register = interpreter.registers.find(reg => reg.register_id === registerId)
      if (!register) return []

      return register.bit_ids
    },

    isInterpreterRegisterBitIncluded: (interpreterId: number, registerId: number, bitId: number): boolean => {
      const interpreter = get().interpreters.find(int => int.interpreter_id === interpreterId)
      if (!interpreter) return false

      const register = interpreter.registers.find(reg => reg.register_id === registerId)
      if (!register) return false

      return register.bit_ids.includes(bitId)
    },

    updateInterpreterNameOrDescription: (interpreterId: number, type: 'name' | 'description', data: string) => {
      set((state) => {
        const interpreter = state.interpreters.find(int => int.interpreter_id === interpreterId)
        if (!interpreter) return

        if (type === 'name') interpreter.name = data
        if (type === 'description') interpreter.description = data
      })
    },

    updateMockBitValue: (interpreterId: number, bitId: number, value: string) => {
      set((state) => {
        const mock = state.mocks.find(mock => mock.interpreter_id === interpreterId)
        const mockIndex = state.mocks.findIndex(mock => mock.interpreter_id === interpreterId)
        if (mockIndex === -1 || !mock) return

        const mockBitIndex = mock.data.findIndex(bit => bit.bit_id === bitId)
        const mockBit = mock.data.find(bit => bit.bit_id === bitId)

        if (mockBitIndex < 0 || !mockBit) {
          mock.data.push({
            bit_id: bitId,
            value,
          })
        }

        else {
          mock.data[mockBitIndex] = {
            bit_id: bitId,
            value
          }
        }
      })
    },

    getMockValue: (interpreterId: number, bitId: number): string => {
      const state = get()

      const mock = state.mocks.find(mock => mock.interpreter_id === interpreterId)
      if (!mock) return "0"

      const data = mock.data.find(data => data.bit_id === bitId)
      if (!data) return "0"

      return data.value
    },

    setSelectedRegister: (id: number, type: SelectedRegisterType) => {
      set((state) => {
        if (type === SelectedRegisterType.CombinedRegister) {
          const selectedReg = state.combinedRegister.find(combReg => combReg.combined_id === id)
          if (!selectedReg) return;
          state.selectedRegister = { id, type, name: selectedReg.name }
        }
        else {
          const selectedReg = state.registers.find(reg => reg.register_id === id)
          if (!selectedReg) return;
          state.selectedRegister = { id, type, name: selectedReg.name }
        }
      })
    },

    setSelectedInterpreter: (interpreterId: number) => {
      set((state) => {
        const interpreter = state.interpreters.find(int => int.interpreter_id === interpreterId)
        if (!interpreter) return;
        state.selectedInterpreter = interpreter
      })
    },

    getIncludedCombinedRegister: (combinedRegisterId: number): RegisterSchema[] => {
      const state = get()
      const combinedRegister = state.combinedRegister.find(combReg => combReg.combined_id === combinedRegisterId);
      if (!combinedRegister) return [];

      const registerIds = combinedRegister.registers.map(reg => reg.register_id)

      return state.registers.filter(reg => registerIds.includes(reg.register_id))
    },

    getCombinedInterpreters: (combinedRegisterId: number): InterpreterSchema[] => {
      const state = get()
      const combinedReg = state.combinedRegister.find(combReg => combReg.combined_id === combinedRegisterId)
      if (!combinedReg) return []

      const combinedInterpreters = state.interpreters.filter(int => int.combined_id === combinedRegisterId && int.type === InterpreterType.CombinedRegister)
      return [...combinedInterpreters]
    }

  }))
)
