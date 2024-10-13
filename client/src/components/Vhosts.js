import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";

export default function Vhosts({ selectedVhost, handleChange, vhosts }) {
  return (
    <FormControl fullWidth>
      <InputLabel id="vhost-select">Select Vhost</InputLabel>
      <Select
        labelId="vhost-select"
        id="vhost-select"
        value={selectedVhost}
        label="Age"
        onChange={handleChange}
      >
        {vhosts.map((vhost, index) => (
          <MenuItem value={vhost.name} key={index}>
            {vhost.name}
          </MenuItem>
        ))}
      </Select>
    </FormControl>
  );
}
