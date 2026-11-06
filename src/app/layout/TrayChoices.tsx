import { useNavigate } from 'react-router-dom';
import { useHostSelection, useTrayChoices } from '@/core/host';

/** Opens what the user picks in the system tray: a Host's Overview, or Add host. */
export function TrayChoices() {
  const navigate = useNavigate();
  const { select } = useHostSelection();
  useTrayChoices({
    onHost: (hostId) => {
      select(hostId);
      void navigate('/');
    },
    onAddHost: () => void navigate('/hosts/new'),
  });
  return null;
}
