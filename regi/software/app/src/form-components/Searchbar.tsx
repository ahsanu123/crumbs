import { Button, InputAdornment, ListItemIcon, MenuItem, MenuList, Stack, TextField, ToggleButton, Tooltip } from "@mui/material"
import { FaSearch } from "react-icons/fa";
import { FaLayerGroup } from "react-icons/fa";

export const Searchbar = () => {
  return (
    <Stack
      direction={'row'}
      spacing={5}
    >

      <TextField
        id="searchbar"
        label="Search"
        placeholder="Example REG0"
        fullWidth
        slotProps={{
          input: {
            startAdornment: (
              <InputAdornment position="start">
                <FaSearch size={34} />
              </InputAdornment>
            ),
          },
        }}
      />

      <Tooltip title="Show Grouped Only">
        <ToggleButton
          value="grouped-only"
        >
          <FaLayerGroup />
        </ToggleButton>
      </Tooltip>

      <Button
        variant="contained"
        aria-label="Search"
      >
        Search
      </Button>


    </Stack>
  )
}
