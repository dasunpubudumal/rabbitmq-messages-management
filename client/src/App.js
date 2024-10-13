import JsonView from "@uiw/react-json-view";
import { useEffect, useState } from "react";

import Box from "@mui/material/Box";
import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";

export default function App() {
  const [vhosts, setVhosts] = useState([]);
  const [selectedVhost, setSelectedVhost] = useState("");

  useEffect(() => {
    async function responses() {
      let response = await fetch("/vhosts");
      setVhosts(await response.json());
    }
    responses();
  }, []);

  const handleChange = (event) => {
    setSelectedVhost(event.target.value);
  };

  return (
    <>
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
      <JsonView value={vhosts} />
    </>
  );
}
