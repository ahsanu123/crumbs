import { Button, FormControl, InputLabel, List, ListItem, ListItemButton, ListItemIcon, ListItemText, MenuItem, Select, Stack, TextField, Typography } from "@mui/material"
import { FaTrash } from "react-icons/fa6";
import { BsMemory } from "react-icons/bs";
import { FaDotCircle } from "react-icons/fa";
import { FaLayerGroup } from "react-icons/fa";
import { SelectedRegisterType, useEditorPageStore } from "../../stores";
import { useMemo } from "react";

export const RegisterDrawer = () => {

  const addNewCombinedRegister = useEditorPageStore(store => store.addNewCombinedRegister)
  const addNewRegister = useEditorPageStore(store => store.addNewRegister)
  const updateCombinedRegister = useEditorPageStore(store => store.updateCombinedRegister)
  const updateRegister = useEditorPageStore(store => store.updateRegister)
  const getCombinedRegisterMembers = useEditorPageStore(store => store.getCombinedRegisterMembers)
  const insertRegisterToCombinedRegister = useEditorPageStore(store => store.insertRegisterToCombinedRegister)
  const setSelectedRegister = useEditorPageStore(store => store.setSelectedRegister)
  const removeCombinedRegister = useEditorPageStore(store => store.removeCombinedRegister)
  const removeCombinedRegisterMember = useEditorPageStore(store => store.removeCombinedRegisterMember)
  const removeRegister = useEditorPageStore(store => store.removeRegister)

  const combinedRegisters = useEditorPageStore(store => store.combinedRegister)
  const registers = useEditorPageStore(store => store.registers)
  const selectedRegister = useEditorPageStore(store => store.selectedRegister)

  const notIncludedRegister = useMemo(() => {
    const allRegisters = combinedRegisters.map(comb => comb.registers).flat()
    const allRegisterIds = allRegisters.map(reg => reg.register_id)

    return registers.filter(reg => !allRegisterIds.includes(reg.register_id))
  }, [registers, combinedRegisters])

  const handleOnAddRegisterToCombinedRegister = (combined_id: number, registerId: number) => {
    insertRegisterToCombinedRegister(combined_id, registerId)
  }

  return (
    <Stack>

      <Stack
        sx={{
          overflow: 'auto',
          maxHeight: '90vh',
          gap: '20px'
        }}
      >
        {combinedRegisters.map((combRegister) => (
          <Stack
            sx={{
              bgcolor: combRegister.combined_id === selectedRegister?.id
                && selectedRegister.type === SelectedRegisterType.CombinedRegister
                ? '#ebf4f6' : ''
            }}
          >
            {combRegister.combined_id === selectedRegister?.id
              && selectedRegister.type === SelectedRegisterType.CombinedRegister
              && (
                <Stack direction='row'>
                  <FaDotCircle />
                  <Typography>
                    Active
                  </Typography>
                </Stack>
              )}
            <Stack
              direction='row'
              sx={{ gap: 5 }}
            >
              <Button
                onClick={() => removeCombinedRegister(combRegister.combined_id)}
                startIcon={<FaTrash />}
              />

              <TextField
                label="Combined Register"
                name={`combined-register-${combRegister.combined_id}`}
                onChange={(event) => updateCombinedRegister(combRegister.combined_id, 'name', event.target.value)}
                key={combRegister.combined_id}
                value={combRegister.name}
                placeholder="combined register name"
              />
              <FormControl sx={{ minWidth: '400px' }}>
                <InputLabel id="register-select">Register</InputLabel>
                <Select
                  labelId="register-select"
                  id="register-select"
                  label="Register To Add"
                  disabled={notIncludedRegister.length === 0}
                  onChange={(ev) => handleOnAddRegisterToCombinedRegister(combRegister.combined_id, parseInt(ev.target.value as string))}
                >
                  {notIncludedRegister.map((reg) => (
                    <MenuItem value={reg.register_id}>{reg.name}</MenuItem>
                  ))}
                </Select>
              </FormControl>
              <Button
                variant='contained'
                onClick={() => setSelectedRegister(combRegister.combined_id, SelectedRegisterType.CombinedRegister)}
              >
                Select
              </Button>
            </Stack>
            <List>
              {getCombinedRegisterMembers(combRegister.combined_id).map(reg => (
                <ListItem
                  secondaryAction={
                    <Button
                      onClick={() => removeCombinedRegisterMember(combRegister.combined_id, reg.register_id)}
                    >
                      <FaTrash />
                    </Button>
                  }
                >
                  <ListItemIcon>
                    <FaDotCircle />
                  </ListItemIcon>

                  <ListItemText
                    primary={reg.name}
                  />
                </ListItem>
              ))}
            </List>
          </Stack>
        ))}

        {notIncludedRegister.map(reg => (
          <Stack
            direction='row'
            sx={{
              bgcolor: reg.register_id === selectedRegister?.id
                && selectedRegister.type === SelectedRegisterType.Register
                ? '#ebf4f6' : ''
            }}
          >
            {reg.register_id === selectedRegister?.id
              && selectedRegister.type === SelectedRegisterType.Register
              && (
                <Stack direction='row'>
                  <FaDotCircle />
                  <Typography>
                    Active
                  </Typography>
                </Stack>
              )}
            <TextField
              label="Register"
              name={`register-${reg.register_id}`}
              key={reg.register_id}
              onChange={event => updateRegister(reg.register_id, 'name', event.target.value)}
              value={reg.name}
              placeholder="register name"
            />

            <Button
              onClick={() => setSelectedRegister(reg.register_id, SelectedRegisterType.Register)}
              variant='contained'
            >
              Select
            </Button>
            <Button
              startIcon={<FaTrash />}
              onClick={() => removeRegister(reg.register_id)}
            />
          </Stack>
        ))}
      </Stack>

      <Stack
        direction='row'
        sx={{ gap: 5 }}
      >
        <Button
          variant="contained"
          startIcon={<BsMemory />}
          onClick={() => addNewCombinedRegister()}
        >
          New Combined Register
        </Button>
        <Button
          variant="contained"
          startIcon={<FaLayerGroup />}
          onClick={() => addNewRegister()}
        >
          New Register
        </Button>
      </Stack>
    </Stack>
  )
}
