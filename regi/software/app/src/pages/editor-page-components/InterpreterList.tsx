import { IconButton, Stack, TextField, Tooltip } from "@mui/material"
import { LiaCashRegisterSolid } from "react-icons/lia";
import { CombinedRegisterSchema } from "../../schema/combined-register"
import { useEditorPageStore } from "../../stores"
import { IoAddCircle } from "react-icons/io5";
import { useMemo } from "react";
import { GiBindle } from "react-icons/gi";
import { InterpreterType } from "../../schema/interpreter";

interface InterpreterListProps {
  combinedRegister: CombinedRegisterSchema
}

export const InterpreterList = (props: InterpreterListProps) => {
  const {
    combinedRegister,
  } = props

  const getCombinedInterpreters = useEditorPageStore(store => store.getCombinedInterpreters)
  const addNewCombinedRegisterInterpreter = useEditorPageStore(store => store.addNewCombinedRegisterInterpreter)
  const updateInterpreterNameOrDescription = useEditorPageStore(store => store.updateInterpreterNameOrDescription)
  const setSelectedInterpreter = useEditorPageStore(store => store.setSelectedInterpreter)
  const interpreters = useEditorPageStore(store => store.interpreters)
  const selectedRegister = useEditorPageStore(store => store.selectedRegister)
  const selectedInterpreter = useEditorPageStore(store => store.selectedInterpreter)

  const combinedInterpreters = useMemo(() => {
    return getCombinedInterpreters(combinedRegister.combined_id)
  }, [interpreters, selectedInterpreter, selectedRegister])

  return (
    <Stack>
      <Stack sx={{ maxWidth: "60px" }}>
        <Tooltip title='Add Interpreter'>
          <IconButton
            onClick={() => addNewCombinedRegisterInterpreter(combinedRegister.combined_id)}
          >
            <IoAddCircle />
          </IconButton>
        </Tooltip>
      </Stack>
      {combinedInterpreters.map((int) => (
        <Stack direction={'row'}>
          <Tooltip title='Select'>
            <IconButton
              onClick={() => setSelectedInterpreter(int.interpreter_id)}
            >
              <LiaCashRegisterSolid />
            </IconButton>
          </Tooltip>
          <TextField
            value={int.name}
            onChange={(event) => updateInterpreterNameOrDescription(int.interpreter_id, 'name', event.target.value)}
          />
          {int.type === InterpreterType.CombinedRegister && (
            <GiBindle />
          )}
        </Stack>
      ))}
    </Stack>
  )
}
