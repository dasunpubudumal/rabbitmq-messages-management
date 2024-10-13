import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";
import { Box, Typography } from "@mui/material";

export default function Vhosts({ selectedVhost, handleChange, vhosts }) {
  return (
    <>
      <Box sx={{ pb: 4 }}>
        <Typography variant="h5" gutterBottom>
          Vhosts
        </Typography>
        <FormControl>
          <Select
            id="vhost-select"
            value={selectedVhost}
            onChange={handleChange}
            style={{ minWidth: 200 }}
            sx={{ backgroundColor: "white" }}
          >
            {vhosts.map((vhost, index) => (
              <MenuItem value={vhost.name} key={index}>
                {vhost.name}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
      </Box>
    </>
  );
}
