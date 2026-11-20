import { Navigate, useLocation } from 'react-router-dom';

/** `/board`, where the Board was before it became the home page: its filters carry over. */
export function OldBoardLink() {
  const { search } = useLocation();
  return <Navigate to={{ pathname: '/', search }} replace />;
}
