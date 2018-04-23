import * as React from 'react';

export class SideForm extends React.Component<any, any> {
  constructor (props: any) {
    super(props);
    this.state = { value: '' };

    this.handleChange = this.handleChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  handleChange (event: any) {
    this.setState({ value: event.target.value });
  }

  handleSubmit (event: any) {
    event.preventDefault();
    this.props.onSubmit(this.state.value);    
  }

  render () {
    return (
      <form onSubmit={this.handleSubmit}>
        <label>
          Side:
          <input type="text" value={this.state.value} onChange={this.handleChange} />
        </label>
        <input type="submit" value="Submit" />
      </form>
    );
  }
}