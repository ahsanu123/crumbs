import { DragDropProvider } from '@dnd-kit/react'
import { Button, MenuItem, Select, Stack, TextField, ToggleButton, ToggleButtonGroup, Typography } from '@mui/material';
import { RegisterSortable } from './RegisterSortable';
import { RegisterSchema } from '../../schema/register';
import { useEditorPageStore } from '../../stores';
import { CombinedRegisterSchema } from '../../schema/combined-register';
import { BitSchema, BitType, bitTypes } from '../../schema/bits';

interface CombinedRegisterListProps {
  combinedRegister: CombinedRegisterSchema
  registers: RegisterSchema[]
}

type RegisterToggleGroupProps = {
  selectedInterpreterId: number
  register: RegisterSchema
}

const RegisterToggleGroup = ({
  selectedInterpreterId,
  register
}: RegisterToggleGroupProps) => {

  const selectedBits = useEditorPageStore(state =>
    state.getInterpreterBits(
      selectedInterpreterId,
      register.register_id
    )
  )

  const updateRegisterBitType = useEditorPageStore(store => store.updateRegisterBitType)
  const updateMockBitValue = useEditorPageStore(store => store.updateMockBitValue)
  const updateRegisterBitResValue = useEditorPageStore(store => store.updateRegisterBitResValue)

  const getMockValue = (interpreterId: number, bitId: number) => {
    const result = useEditorPageStore(store => store.getMockValue(interpreterId, bitId))
    return result
  }

  return (
    <Stack>
      <Typography>{register.name}</Typography>

      <ToggleButtonGroup value={selectedBits}>
        <Stack direction='row-reverse'>
          {register.bits.map((bit) => (
            <BitToggle
              key={bit.bit_id}
              selectedInterpreterId={selectedInterpreterId}
              registerId={register.register_id}
              bit={bit}
            />
          ))}
        </Stack>
      </ToggleButtonGroup>
      <Stack direction='row-reverse'>
        {register.bits.map((bit, index) => (
          <Stack key={index}>
            <TextField
              name={`bit ${bit.bit_ordinal} mock`}
              label={`bit ${bit.bit_ordinal} mock`}
              disabled={!selectedBits.includes(bit.bit_id)}
              type='number'
              value={getMockValue(selectedInterpreterId, bit.bit_id)}
              onChange={(event) => updateMockBitValue(selectedInterpreterId, bit.bit_id, event.target.value)}
              slotProps={{
                htmlInput: { min: 0, max: 1 }
              }}
            />
            <TextField
              fullWidth
              id={`bits-resval-${index}`}
              label="ResVal"
              type="number"
              value={bit.reset_val}
              disabled={!selectedBits.includes(bit.bit_id)}
              onChange={(e) => updateRegisterBitResValue(register.register_id, bit.bit_id, e.target.value)}
              slotProps={{
                htmlInput: { min: 0, max: 1, step: 1 }
              }}
            />
            <Select
              labelId="interpreter-types"
              id="interpreter-types"
              label="Interpreter Type"
              value={bit.bit_type}
              disabled={!selectedBits.includes(bit.bit_id)}
              onChange={(e) => updateRegisterBitType(register.register_id, bit.bit_id, e.target.value as BitType)}
            >
              {bitTypes.map((intType) => (
                <MenuItem value={intType}>{intType}</MenuItem>
              ))}
            </Select>
          </Stack>
        ))}
      </Stack>
    </Stack>
  )
}
type BitToggleProps = {
  selectedInterpreterId: number
  registerId: number
  bit: BitSchema
}

const BitToggle = ({
  selectedInterpreterId,
  registerId,
  bit
}: BitToggleProps) => {

  const updateInterpreterBits = useEditorPageStore(state => state.updateInterpreterBits)

  return (
    <Stack>
      <ToggleButton
        fullWidth
        value={bit.bit_id}
        onChange={() =>
          updateInterpreterBits(
            selectedInterpreterId,
            registerId,
            bit.bit_id
          )
        }
      >
        {`Bit ${bit.bit_ordinal}`}
      </ToggleButton>
    </Stack>
  )
}

export const CombinedRegisterList = (props: CombinedRegisterListProps) => {
  const {
    combinedRegister,
    registers
  } = props

  const moveCombinedRegister = useEditorPageStore(store => store.moveCombinedRegister)
  const removeCombinedRegisterMember = useEditorPageStore(store => store.removeCombinedRegisterMember)
  const updateCombinedRegister = useEditorPageStore(store => store.updateCombinedRegister)
  const getOrderedCombinedRegisterMember = useEditorPageStore(store => store.getOrderedCombinedRegisterMember)

  const orderedCombinedRegisterMember = getOrderedCombinedRegisterMember(combinedRegister.combined_id)

  const selectedInterpreter = useEditorPageStore(store => store.selectedInterpreter)

  return (
    <Stack>
      <Stack
        sx={{
          margin: "30px 0 10px 0",
          gap: "30px"
        }}
      >
        <TextField
          value={combinedRegister.name}
          label="combined register name"
          placeholder="create combined register name"
          onChange={(ev) => updateCombinedRegister(combinedRegister.combined_id, 'name', ev.target.value)}
        />

        <DragDropProvider
          onDragEnd={(ev) => moveCombinedRegister(combinedRegister.combined_id, ev)}
        >
          <Stack
            direction={'row'}
            sx={{
              backgroundColor: '#e8eaf6',
              borderRadius: "10px",
              border: "1px solid gray",
              minHeight: "50px"
            }}
          >
            {registers.length === 0 && (
              <Typography
                sx={{
                  alignSelf: 'center',
                  marginLeft: "20px"
                }}
              >
                Empty, please select register to combine.
              </Typography>
            )}

            {registers.map((reg, index) =>
              <RegisterSortable
                key={reg.register_id}
                register={reg}
                index={index}
                onRemoveRegister={(regId) => removeCombinedRegisterMember(combinedRegister.combined_id, regId)}
              />
            )}
          </Stack>
        </DragDropProvider>
      </Stack>

      <Stack
        direction={'row'}
        sx={{
          justifyContent: 'space-between',
          margin: "0 0 25px 0"
        }}
      >
        <Typography
          variant="subtitle2"
        >
          MSB
        </Typography>

        <Typography
          variant="subtitle2"
        >
          LSB
        </Typography>
      </Stack>

      {selectedInterpreter && combinedRegister.combined_id === selectedInterpreter.combined_id && (
        <Stack>
          <Typography>
            {selectedInterpreter.name}
          </Typography>
          {orderedCombinedRegisterMember.map((register) => (
            <RegisterToggleGroup
              key={register.register_id}
              selectedInterpreterId={selectedInterpreter.interpreter_id}
              register={register}
            />
          ))}
        </Stack>
      )}

      <Typography>
        Result: .....
      </Typography>

      <TextField
        id="formula-input"
        label="Formula"
        size="medium"
        variant="standard"
        placeholder="editable formula"
        fullWidth
        multiline
        rows={4}
      />

      <Stack
        direction={'row'}
        sx={{
          marginTop: "30px",
          justifyContent: 'space-between',
        }}
      >
        <Button
          variant="contained"
        >
          Evaluate
        </Button>

        <Typography
          variant="body1"
          color="error"
        >
          <b>
            Error: error message
          </b>
        </Typography>

      </Stack>
    </Stack>
  );
}
