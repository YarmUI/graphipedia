import Autocomplete from '@mui/material/Autocomplete';
import TextField from '@mui/material/TextField';
import useSearchPageResult from '../hooks/useSearchPageResult';
import { useState, useRef } from 'react';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';

export default ({label, value, setValue}: {label?: string, value?: string, setValue: (value: string) => void}) => {
  const [query, setQuery] = useState('');
  const { data, loading } = useSearchPageResult({ query: query, limit: 5 });
  const debounceRef = useRef<number | null>(null);

  const handleInputChange = (_: any, v: string) => {
    setValue(v);
    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }

    debounceRef.current = window.setTimeout(() => {
      setQuery(v);
    }, 300);
  };

  const handleChange = (_: any, v: any | null) => {
    if(typeof v === 'object' && v !== null && 'title' in v) {
      setValue(v.title);
    } else if (typeof v === 'string') {
      setValue(v);
    } else {
      setValue('');
    }
  };

  return (
    <Autocomplete
      freeSolo={true}
      options={data?.items || []}
      loading={loading}
      autoHighlight
      inputValue={value}
      value={value}
      onChange={handleChange}
      onInputChange={handleInputChange}
      getOptionLabel={(option) => {
        if(typeof option === 'object' && option !== null && 'title' in option) {
          return option.title;
        }
        return option || "";
      }}
      sx={{
        width: {
          xs: '100%',
          sm: '100%',
          md: 400,
          lg: 400,
        }
      }}
      renderInput={(params) => (
        <TextField
          {...params}
          label={label}
        />
      )}
      renderOption={(props, option) => {
        const { key, ...optionProps } = props;
        return (
          <li {...optionProps} key={key}>
            <Box sx={{ display: 'flex', flexDirection: 'column', width: '100%' }}>
              <Typography variant="subtitle1">
                {option.title}
              </Typography>
              {option.is_redirect && option.redirected_title && (
                <Typography variant="body2" color="text.secondary" sx={{ mt: 0.5 }}>
                  （{option.redirected_title} にリダイレクト）
                </Typography>
              )}
              <Typography variant="caption" color="text.secondary">
                {option.link_count} links
              </Typography>
            </Box>
          </li>
        );
      }}
    />
  );
}