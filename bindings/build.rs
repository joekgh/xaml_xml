winrt::build!(
    dependencies
        os
    types
        windows::ui::xaml::hosting::{DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory, WindowsXamlManager}
        windows::ui::xaml::media::SolidColorBrush
        windows::ui::xaml::controls::{StackPanel, IStackPanelFactory, TextBlock, TextBox, ITextBoxFactory}
        windows::ui::{Color, Colors}
        windows::foundation::PropertyValue
        windows::ui::xaml::markup::XamlReader
        windows::ui::xaml::shapes::Ellipse
);

fn main() {
    build()
}