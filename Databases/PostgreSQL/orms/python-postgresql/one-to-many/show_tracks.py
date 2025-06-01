import click


@click.command("show-tracks")
@click.pass_context  # Decorate 'version_command' to receive context
def show_tracks(ctx):  # Add 'ctx' as the first parameter
    """Shows all tracks."""
    click.echo("App Version: 1.0.0")

    # Example: Accessing data from ctx.obj set by the main() function
    if 'DB_HOST' in ctx.obj and ctx.obj['DB_HOST']:
        click.echo(f"Current configuration is targeting DB host: {ctx.obj['DB_HOST']}")
    else:
        click.echo("DB host information not available in context or not set.")

    if ctx.obj.get('ENV_LOADED_SUCCESSFULLY'):
        click.echo(f"Environment was loaded from: {ctx.obj.get('ENV_FILE_PATH')}")
    else:
        click.echo(f"Attempted to load environment from: {ctx.obj.get('ENV_FILE_PATH')} (but failed or file not found)")