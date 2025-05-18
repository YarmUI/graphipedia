'use client'

import Autocomplete from '@mui/material/Autocomplete';
import TextField from '@mui/material/TextField';
import { useState, useRef } from 'react';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';
import { components } from '../../types/api-types';
import useSWR from 'swr';
import fetchTitleSearch from '@/lib/fetchTitleSearch';

type TitleSearchResultItem = components['schemas']['TitleSearchResultItem'];
type TitleSearchResult = components['schemas']['TitleSearchResult'];

export default ({
  label,
  value,
  setValue
}: {
  label?: string,
  value: TitleSearchResultItem | null,
  setValue?: (value: TitleSearchResultItem | null) => void
}) => {
  const [inputValue, setInputValue] = useState('');
  const debounceRef = useRef<number | null>(null);

  const { data } = useSWR<TitleSearchResult>(
    inputValue.length > 0 ? ['titleSearch', inputValue] : null,
    () => fetchTitleSearch({ query: inputValue, limit: 5 }),
  );

  const handleInputChange = (_: any, v: string) => {
    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }

    debounceRef.current = window.setTimeout(() => {
      setInputValue(v);
    }, 200);
  }

  const handleChange = (_: any, v: TitleSearchResultItem | null) => {
    v && setValue && setValue(v);
  }

  return (
    <Autocomplete
      autoHighlight
      autoSelect
      options={data?.items || []}
      onInputChange={handleInputChange}
      onChange={handleChange}
      value={value}
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
      getOptionLabel={option => option.title }
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
  )
}