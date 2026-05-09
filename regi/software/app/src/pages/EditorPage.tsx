import { Stack } from "@mui/material"
import { RegisterDrawer } from "./editor-page-components/RegisterDrawer"
import { SelectedRegisterType, useEditorPageStore } from "../stores"
import { CombinedRegisterList } from "./editor-page-components"
import { useMemo } from "react"

export const EditorPage = () => {

  const selectedRegister = useEditorPageStore(store => store.selectedRegister)
  const getIncludedCombinedRegister = useEditorPageStore(store => store.getIncludedCombinedRegister)

  const combinedRegisters = useEditorPageStore(store => store.combinedRegister)
  const registers = useEditorPageStore(store => store.registers)

  const selectedCombinedRegister = useMemo(() => {
    if (selectedRegister && selectedRegister.type === SelectedRegisterType.CombinedRegister) {
      return combinedRegisters.find(combReg => combReg.combined_id === selectedRegister.id)
    }
    return undefined
  }, [selectedRegister, combinedRegisters])

  const includedCombinedRegister = useMemo(() => {
    if (!selectedRegister || selectedRegister?.type === SelectedRegisterType.Register) return []
    return getIncludedCombinedRegister(selectedRegister.id)
  }, [selectedRegister, combinedRegisters, registers])

  return (
    <Stack direction='row'>
      <RegisterDrawer />

      {selectedCombinedRegister && (
        <CombinedRegisterList
          combinedRegister={selectedCombinedRegister}
          registers={includedCombinedRegister}
        />
      )}
    </Stack>
  )
}
