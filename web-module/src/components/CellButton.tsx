import * as React from 'react';

import './CellButton.css';

export class CellButton extends React.Component<any, any> {
  constructor (props: any) {
    super(props);
    this.state = { isAlive: false };
  }
  click = (e: any) => {
    this.props.onClick(this);    
    this.setState((prevState: any, props: any) => ({
      isAlive: !prevState.isAlive
    }));
  }
  render () {
    const btnClass = this.state.isAlive ? 'cell-btn alive' : 'cell-btn dead';

    return <button className={btnClass} onClick={this.click} />
  }
}
