import React, { useEffect, useState } from 'react';
import ServiceDetails from './ServiceDetails';

const ServiceSidebar: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [serviceId, setServiceId] = useState<string | null>(null);

  useEffect(() => {
    const openSidebar = (e: CustomEvent) => {
      setServiceId(e.detail.serviceId);
      setIsOpen(true);
    };

    const closeSidebar = () => {
      setIsOpen(false);
    };

    // Listen for custom events from the Astro components
    document.addEventListener('openServiceSidebar', openSidebar as EventListener);
    document.addEventListener('closeServiceSidebar', closeSidebar as EventListener);

    return () => {
      document.removeEventListener('openServiceSidebar', openSidebar as EventListener);
      document.removeEventListener('closeServiceSidebar', closeSidebar as EventListener);
    };
  }, []);

  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        setIsOpen(false);
      }
    };

    document.addEventListener('keydown', handleEscape);
    return () => document.removeEventListener('keydown', handleEscape);
  }, [isOpen]);

  useEffect(() => {
    if (isOpen) {
      // Prevent body scrolling
      document.body.style.overflow = 'hidden';
    } else {
      // Restore body scrolling
      document.body.style.overflow = '';
    }

    return () => {
      document.body.style.overflow = '';
    };
  }, [isOpen]);

  const handleClose = () => {
    setIsOpen(false);
  };

  if (!isOpen) return null;

  return (
    <>
      {/* Sidebar Overlay */}
      <div 
        className="z-40 fixed inset-0 bg-black/50 opacity-100 backdrop-blur-sm transition-opacity duration-300"
        onClick={handleClose}
      />
      
      {/* Service Details Sidebar */}
      <div 
        className="right-0 z-50 fixed inset-y-0 bg-black/95 backdrop-blur-xl border-white/10 border-l w-1/2 overflow-hidden transition-transform translate-x-0 duration-300 transform"
      >
        <div className="flex flex-col h-full">
          <div className="flex justify-between items-center p-6 border-white/10 border-b">
            {serviceId && (
              <h2 className="font-bold text-white text-2xl">
                {serviceId === 'dev' && 'Web & Mobile Development'}
                {serviceId === 'design' && 'Design & Branding'}
                {serviceId === 'motion' && 'Motion & Content'}
              </h2>
            )}
            <button 
              title='sidebar-button'
              onClick={handleClose}
              className="text-white/60 hover:text-white transition-colors"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>
          <div className="flex-1 p-6 overflow-y-auto">
            {serviceId && <ServiceDetails serviceId={serviceId} />}
          </div>
        </div>
      </div>
    </>
  );
};

export default ServiceSidebar;
